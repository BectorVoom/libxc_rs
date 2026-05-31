//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1225/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1225<F: Float>(t10940: F, t11483: F, t3275: F, t3582: F, t37543: F, t11855: F, t1561: F, t3277: F, t3262: F, t3574: F, t37318: F, t113: F, t3578: F, t97: F) -> (F, F, F, F, F) {
    let t40699 = t10940 * t11483 / F::cast_from(4.0_f64);
    let t40704 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t37543 * t3582;
    let t40705 = t1561 * t11855;
    let t40708 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t40705 * t3277;
    let t40711 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t37318 * t3574;
    let t40713 = t97 * t3578 * t113;
    (t40699, t40704, t40708, t40711, t40713)
}
