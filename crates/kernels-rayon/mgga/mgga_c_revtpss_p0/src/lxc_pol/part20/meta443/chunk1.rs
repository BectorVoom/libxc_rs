//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1695/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1695(t2350: f64, t2256: f64, t36415: f64, t2358: f64, t2362: f64, t100: f64, t101: f64, t10217: f64, t10227: f64, t10229: f64, t10232: f64, t10233: f64, t10236: f64, t10237: f64, t10241: f64, t10246: f64, t10250: f64, t10344: f64, t105: f64, t108: f64, t2344: f64, t2349: f64, t2351: f64, t2354: f64, t2357: f64, t46173: f64, t46196: f64, t656: f64, t659: f64, t97: f64, tau0: f64) -> f64 {
    let t46197 = t2350 * t2350;
    let t46205 = t2256 * t2256;
    let t46212 = 1.0_f64 / t36415;
    let t46213 = t2358 * t2358;
    let t46221 = t2362 * t2362;
    let t46228 = 5.0_f64 / 3.0_f64 * t105 * t108 * t46173 + 6160.0_f64 / 81.0_f64 * tau0 * t10344 * t101 - 8800.0_f64 / 81.0_f64 * t10217 * t659 + 400.0_f64 / 9.0_f64 * t2344 * t2354 - 100.0_f64 / 9.0_f64 * t656 * t10237 - 5.0_f64 / 3.0_f64 * t97 * t100 * t46173 + 800.0_f64 / 27.0_f64 * t2344 * t2351 + 200.0_f64 / 81.0_f64 * t656 * t10229 - 200.0_f64 / 9.0_f64 * t656 * t10233 + 40.0_f64 / 81.0_f64 * t97 * t46196 * t46197 - 20.0_f64 / 9.0_f64 * t97 * t10227 * t2350 * t2256 + 10.0_f64 / 3.0_f64 * t97 * t2349 * t46205 + 40.0_f64 / 9.0_f64 * t97 * t10232 * t10236 + 40.0_f64 / 81.0_f64 * t105 * t46212 * t46213 - 20.0_f64 / 9.0_f64 * t105 * t10241 * t2358 * t2362 + 10.0_f64 / 3.0_f64 * t105 * t2357 * t46221 + 40.0_f64 / 9.0_f64 * t105 * t10246 * t10250;
    t46228
}
