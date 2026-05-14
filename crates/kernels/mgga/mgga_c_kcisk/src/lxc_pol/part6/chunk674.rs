//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 674/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk674<F: Float>(t143: F, t3532: F, t1390: F, t213: F, t3830: F, t423: F, t394: F, t4143: F, t10471: F, t140: F, t416: F, t382: F, t3783: F, t1457: F, t475: F, t13328: F, t484: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14093 = t143 * t3532;
    let t14100 = t213 * t1390;
    let t14140 = 1.0 / t3830 / t423;
    let t14208 = t394 * t4143;
    let t14223 = t140 * t10471 * t416;
    let t14255 = t382 * t3532;
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14292 = t1457 * t1457;
    let t14293 = 1.0 / t14292;
    let t14294 = t475 * t14293;
    let t14364 = t484 * t13328;
    (t14093, t14100, t14140, t14208, t14223, t14255, t14265, t14294, t14364)
}
