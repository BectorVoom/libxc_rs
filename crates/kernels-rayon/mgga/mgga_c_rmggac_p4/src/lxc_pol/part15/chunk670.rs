//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 670/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk670(t8881: f64, t8885: f64, t9047: f64, t9071: f64, t9073: f64, t7910: f64, t9124: f64, t9126: f64, t9129: f64, t9148: f64, t9223: f64, t9225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9600 = 0.2993560425465952141e-1_f64 * t8881;
    let t9601 = 0.8980681276397856423e-1_f64 * t8885;
    let t9605 = 0.1064114997332445985e-4_f64 * t9047;
    let t9613 = 0.5987120850931904282e-1_f64 * t9071;
    let t9614 = 0.5987120850931904282e-1_f64 * t9073;
    let t9631 = 0.59590439850616975158e-4_f64 * t7910;
    let t9636 = 0.1064114997332445985e-4_f64 * t9124;
    let t9646 = 0.2993560425465952141e-1_f64 * t9126;
    let t9647 = 0.5987120850931904282e-1_f64 * t9129;
    let t9653 = 0.1064114997332445985e-4_f64 * t9148;
    let t9670 = 0.1064114997332445985e-4_f64 * t9223;
    let t9671 = 0.8980681276397856423e-1_f64 * t9225;
    (t9600, t9601, t9605, t9613, t9614, t9631, t9636, t9646, t9647, t9653, t9670, t9671)
}
