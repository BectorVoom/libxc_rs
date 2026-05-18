//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1038/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1038<F: Float>(t11248: F, t1934: F, t1672: F, t2920: F, t2923: F, t6611: F, t1904: F, t2021: F, t2779: F, t2772: F, t11238: F, t11243: F, t11246: F, t6613: F, t6616: F, t6622: F, t6624: F, t6625: F, t6628: F, t6630: F, t6662: F, t6665: F) -> F {
    let t11249 = t1934 * t11248;
    let t11251 = t2920 * t1672;
    let t11253 = t6611 * t2923;
    let t11254 = t1904 * t11253;
    let t11257 = t2779 * t2021;
    let t11259 = t2772 * t1672;
    let t11261 = F::new(0.8091720650647759) * t11238 - F::new(0.013716887843283197) * t6613 + F::new(0.013716887843283197) * t6616 - t6622 + t6624 + F::new(6.211752672544321) * t6625 + F::new(7.35994946043302) * t11243 - t6628 + t6630 + F::new(0.013716887843283197) * t11246 + F::new(1.6457779058161184) * t11249 - F::new(1.6457779058161184) * t11251 - F::new(0.013716887843283197) * t11254 + F::new(4.738783832122567) * t6662 + F::new(1.2536914064583544) * t11257 + F::new(1.2536914064583544) * t11259 + t6665;
    t11261
}
