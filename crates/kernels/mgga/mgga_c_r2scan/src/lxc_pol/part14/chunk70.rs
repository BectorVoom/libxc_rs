//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 70/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk70<F: Float>(t149: F, t78: F, t79: F, t80: F) -> (F, F, F) {
    let t212 = F::cast_from(0.258925e1_f64) * t149 + t78 + t79 + t80;
    let t215 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t212;
    let t216 = F::ln(t215);
    (t212, t215, t216)
}
