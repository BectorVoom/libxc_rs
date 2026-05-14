//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 930/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk930<F: Float>(t2333: F, t3492: F, t3718: F, t2332: F, t6660: F, t815: F, t312: F, t320: F, t6659: F, t325: F, t326: F, t6691: F, t1337: F, t1347: F, t260: F, t277: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14402 = t2333 * t3492;
    let t15059 = t2333 * t3718;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0 / t19025;
    let t19146 = t815 * t6660;
    let t19155 = t312 / t6659 / t320;
    let t19203 = t325 / t6691 / t326;
    let t19309 = 1.0 / t1347 / t1337;
    let t19326 = t1347 * t1347;
    let t19327 = 1.0 / t19326;
    let t19790 = t260 * t277;
    (t14402, t15059, t19026, t19146, t19155, t19203, t19309, t19327, t19790)
}
