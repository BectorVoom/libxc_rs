//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 32/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk32<F: Float>(t12: F, t18: F, t26: F, t15: F) -> (F, F, F, F, F, F) {
    let t78 = F::cast_from(0.905775e0_f64) * t12;
    let t79 = F::cast_from(0.1100325e0_f64) * t18;
    let t80 = F::cast_from(0.1241775e0_f64) * t26;
    let t81 = F::cast_from(0.51785e1_f64) * t15 + t78 + t79 + t80;
    let t84 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t81;
    let t85 = F::ln(t84);
    (t78, t79, t80, t81, t84, t85)
}
