//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 63/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk63<F: Float>(t14: F, t182: F, t12: F, t168: F) -> (F, F, F) {
    let t183 = t14 * t182;
    let t185 = F::sqrt(t12);
    let t188 = F::cast_from(0.379785e1_f64) * t168 + F::cast_from(0.35876e1_f64) + F::cast_from(0.122865e1_f64) * t185 + F::cast_from(0.24647e0_f64) * t12;
    (t183, t185, t188)
}
