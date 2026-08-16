//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1139/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1139(t40044: f64, t40047: f64, t40050: f64, t40053: f64, t40076: f64, t40086: f64, t40090: f64, t40102: f64, t40131: f64, t40155: f64, t40157: f64, t40162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41669 = 0.27944763721877274748e0_f64 * t40044;
    let t41670 = 0.93149212406257582492e-1_f64 * t40047;
    let t41671 = 0.27944763721877274748e0_f64 * t40050;
    let t41672 = 0.93149212406257582492e-1_f64 * t40053;
    let t41682 = 0.95219938395347901946e-2_f64 * t40076;
    let t41687 = 0.93149212406257582492e-1_f64 * t40086;
    let t41689 = 0.11177905488750909899e1_f64 * t40090;
    let t41694 = 0.39029762157531132074e-1_f64 * t40102;
    let t41709 = 0.18629842481251516498e0_f64 * t40131;
    let t41721 = 0.93149212406257582492e-1_f64 * t40155;
    let t41722 = 0.46230515946956099004e0_f64 * t40157;
    let t41725 = 0.27944763721877274748e0_f64 * t40162;
    (t41669, t41670, t41671, t41672, t41682, t41687, t41689, t41694, t41709, t41721, t41722, t41725)
}
