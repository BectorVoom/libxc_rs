//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1090/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1090<F: Float>(t10940: F, t11483: F, t3275: F, t3582: F, t37543: F, t11855: F, t1561: F, t3277: F, t3262: F, t3574: F, t37318: F, t113: F, t3578: F, t97: F, t10986: F, t10635: F, t40282: F) -> (F, F, F, F, F, F) {
    let t40699 = t10940 * t11483 / 4.0;
    let t40704 = 5.0 / 16.0 * t3275 * t37543 * t3582;
    let t40705 = t1561 * t11855;
    let t40708 = 5.0 / 8.0 * t3275 * t40705 * t3277;
    let t40711 = 3.0 / 4.0 * t3262 * t37318 * t3574;
    let t40713 = t97 * t3578 * t113;
    let t40715 = 5.0 / 8.0 * t40713 * t10986;
    let t40717 = 15.0 / 8.0 * t40282 * t10635;
    (t40699, t40704, t40708, t40711, t40715, t40717)
}
