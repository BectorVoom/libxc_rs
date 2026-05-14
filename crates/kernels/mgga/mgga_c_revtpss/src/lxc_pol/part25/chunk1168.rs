//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1168/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1168<F: Float>(t25880: F, t94403: F, t25904: F, t25945: F, t9285: F, t25944: F, t1364: F, t26075: F, t786: F, t2022: F, t9898: F, t26028: F, t9807: F, t9812: F, t2482: F, t7262: F, t814: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94404 = t25880 * t94403;
    let t94405 = t25904 * t94404;
    let t94407 = t25945 * t9285;
    let t94409 = 0.68540937416128198417e-2 * t25944 * t94407;
    let t94411 = t786 * t26075 * t1364;
    let t94413 = t2022 * t9898;
    let t94418 = t26028 * t9807;
    let t94420 = t26028 * t9812;
    let t94423 = t2482 * t7262 * t814;
    (t94404, t94405, t94407, t94409, t94411, t94413, t94418, t94420, t94423)
}
