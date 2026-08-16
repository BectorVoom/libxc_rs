//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1038/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1038(t11248: f64, t1934: f64, t1672: f64, t2920: f64, t2923: f64, t6611: f64, t1904: f64, t2021: f64, t2779: f64, t2772: f64, t11238: f64, t11243: f64, t11246: f64, t6613: f64, t6616: f64, t6622: f64, t6624: f64, t6625: f64, t6628: f64, t6630: f64, t6662: f64, t6665: f64) -> f64 {
    let t11249 = t1934 * t11248;
    let t11251 = t2920 * t1672;
    let t11253 = t6611 * t2923;
    let t11254 = t1904 * t11253;
    let t11257 = t2779 * t2021;
    let t11259 = t2772 * t1672;
    let t11261 = 0.8091720650647759_f64 * t11238 - 0.013716887843283197_f64 * t6613 + 0.013716887843283197_f64 * t6616 - t6622 + t6624 + 6.211752672544321_f64 * t6625 + 7.35994946043302_f64 * t11243 - t6628 + t6630 + 0.013716887843283197_f64 * t11246 + 1.6457779058161184_f64 * t11249 - 1.6457779058161184_f64 * t11251 - 0.013716887843283197_f64 * t11254 + 4.738783832122567_f64 * t6662 + 1.2536914064583544_f64 * t11257 + 1.2536914064583544_f64 * t11259 + t6665;
    t11261
}
