//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 834/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk834(t3437: f64, t6720: f64, t388: f64, t6613: f64, t387: f64, t1187: f64, t3346: f64, t6496: f64, t382: f64, t358: f64, t6555: f64, t6683: f64, t6685: f64, t6687: f64, t6691: f64, t6694: f64, t6698: f64, t6702: f64, t6706: f64, t6710: f64, t6712: f64, t6714: f64, t6718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6721 = t3437 * t6720;
    let t6723 = t388 * t6613;
    let t6724 = t387 * t6723;
    let t6725 = t1187 * t6724;
    let t6727 = t3346 * t6496;
    let t6728 = t387 * t6727;
    let t6729 = t382 * t6728;
    let t6731 = t358 * t6555;
    let t6732 = t387 * t6731;
    let t6733 = t382 * t6732;
    let t6735 = t6683 / 16.0_f64 - t6685 / 8.0_f64 + t6687 / 12.0_f64 + t6691 / 8.0_f64 - t6694 / 12.0_f64 - t6698 / 16.0_f64 - t6702 / 72.0_f64 + t6706 / 24.0_f64 - t6710 / 256.0_f64 + t6712 / 128.0_f64 - t6714 / 96.0_f64 - t6718 / 128.0_f64 + t6721 / 96.0_f64 + t6725 / 256.0_f64 - t6729 / 576.0_f64 - t6733 / 192.0_f64;
    (t6721, t6723, t6724, t6725, t6727, t6728, t6729, t6731, t6732, t6733, t6735)
}
