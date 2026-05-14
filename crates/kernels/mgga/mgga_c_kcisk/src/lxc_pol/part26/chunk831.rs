//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 831/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk831<F: Float>(t1056: F, t14100: F, t1387: F, t3830: F, t423: F, t1407: F, t3805: F, t1404: F, t3783: F, t1299: F, t3795: F, t394: F, t4143: F, t1284: F, t10471: F, t140: F, t416: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14101 = t14100 * t1056;
    let t14107 = t1387 * t1056;
    let t14140 = 1.0 / t3830 / t423;
    let t14160 = t3805 * t1407;
    let t14187 = t1404 * t3783;
    let t14188 = t14187 * sigma0;
    let t14199 = t3795 * t1299;
    let t14208 = t394 * t4143;
    let t14213 = t1299 * t1284;
    let t14223 = t140 * t10471 * t416;
    (t14101, t14107, t14140, t14160, t14187, t14188, t14199, t14208, t14213, t14223)
}
