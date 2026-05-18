//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1058/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1058<F: Float>(t1211: F, t12646: F, t1214: F, t3790: F, t1277: F, t3552: F, t487: F, t1208: F, t3551: F, t1210: F, t1215: F, t12600: F, t12603: F, t12607: F, t12622: F, t12628: F, t12630: F, t12633: F, t12641: F, t1295: F, t3556: F, t3567: F, t3569: F, t3572: F, t3576: F, t3585: F, t3732: F, t3791: F) -> (F, F) {
    let t12647 = t1211 * t12646;
    let t12650 = t1214 * t3790;
    let t12651 = t1277 * t12650;
    let t12654 = t3552 * t487;
    let t12657 = t3551 * t1208;
    let t12658 = t12657 * t487;
    let t12663 = -F::new(0.39512695097613069591e1) * t3567 * t12600 - F::new(0.39512695097613069591e1) * t12603 * t1295 + F::new(0.19756347548806534796e1) * t1210 * t12607 - F::new(0.65854491829355115987e0) * t1210 * t12622 - F::new(0.39512695097613069591e1) * t12628 * t12630 + F::new(0.39512695097613069591e1) * t12633 * t3569 + F::new(0.39512695097613069591e1) * t3572 * t3576 - F::new(0.19756347548806534796e1) * t3572 * t3585 + F::new(0.39512695097613069591e1) * t12641 * t3569 + F::new(0.39512695097613069591e1) * t3556 * t3576 + F::new(0.39512695097613069591e1) * t3567 * t12647 + F::new(0.19756347548806534796e1) * t1210 * t12651 - F::new(0.19756347548806534796e1) * t12654 * t1295 - F::new(0.19756347548806534796e1) * t12658 * t1215 - F::new(0.19756347548806534796e1) * t3732 * t3791;
    (t12657, t12663)
}
