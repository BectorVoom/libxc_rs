//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 939/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk939<F: Float>(t12657: F, t487: F, t1210: F, t1215: F, t12600: F, t12603: F, t12607: F, t12622: F, t12628: F, t12630: F, t12633: F, t12641: F, t12647: F, t12651: F, t12654: F, t1295: F, t3556: F, t3567: F, t3569: F, t3572: F, t3576: F, t3585: F, t3732: F, t3791: F) -> (F,) {
    let t12658 = t12657 * t487;
    let t12663 = -0.39512695097613069591e1 * t3567 * t12600 - 0.39512695097613069591e1 * t12603 * t1295 + 0.19756347548806534796e1 * t1210 * t12607 - 0.65854491829355115987e0 * t1210 * t12622 - 0.39512695097613069591e1 * t12628 * t12630 + 0.39512695097613069591e1 * t12633 * t3569 + 0.39512695097613069591e1 * t3572 * t3576 - 0.19756347548806534796e1 * t3572 * t3585 + 0.39512695097613069591e1 * t12641 * t3569 + 0.39512695097613069591e1 * t3556 * t3576 + 0.39512695097613069591e1 * t3567 * t12647 + 0.19756347548806534796e1 * t1210 * t12651 - 0.19756347548806534796e1 * t12654 * t1295 - 0.19756347548806534796e1 * t12658 * t1215 - 0.19756347548806534796e1 * t3732 * t3791;
    (t12663,)
}
