//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 889/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk889<F: Float>(t11325: F, t3275: F, t3277: F, t3570: F, t860: F, t1146: F, t2337: F, t10985: F, t3472: F, t1114: F, t2333: F) -> (F, F, F, F, F, F) {
    let t11327 = t3275 * t11325 * t3277;
    let t11328 = 5.0 / 8.0 * t11327;
    let t11329 = t860 * t3570;
    let t11330 = 2.0 * t11329;
    let t11331 = t1146 * t2337;
    let t11333 = t3472 * t10985;
    let t11334 = t3275 * t11333;
    let t11335 = 5.0 / 8.0 * t11334;
    let t11336 = t1114 * t2333;
    (t11328, t11330, t11331, t11333, t11335, t11336)
}
