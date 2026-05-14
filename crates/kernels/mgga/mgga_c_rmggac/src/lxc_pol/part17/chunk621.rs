//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 621/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk621<F: Float>(t9047: F, t9071: F, t9073: F, t7910: F, t9124: F, t9126: F, t9129: F, t9148: F, t9223: F, t9225: F, t9229: F, t2347: F, t570: F, t262: F, t7204: F, t558: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9605 = 0.1064114997332445985e-4 * t9047;
    let t9613 = 0.5987120850931904282e-1 * t9071;
    let t9614 = 0.5987120850931904282e-1 * t9073;
    let t9631 = 0.59590439850616975158e-4 * t7910;
    let t9636 = 0.1064114997332445985e-4 * t9124;
    let t9646 = 0.2993560425465952141e-1 * t9126;
    let t9647 = 0.5987120850931904282e-1 * t9129;
    let t9653 = 0.1064114997332445985e-4 * t9148;
    let t9670 = 0.1064114997332445985e-4 * t9223;
    let t9671 = 0.8980681276397856423e-1 * t9225;
    let t9672 = 0.5987120850931904282e-1 * t9229;
    let t9704 = t2347 * t570;
    let t9705 = t262 * t9704;
    let t9706 = t7204 * t9705;
    let t9707 = 0.20455996240684006296e-1 * t9706;
    let t9708 = t2347 * t558;
    (t9605, t9613, t9614, t9631, t9636, t9646, t9647, t9653, t9670, t9671, t9672, t9704, t9705, t9707, t9708)
}
