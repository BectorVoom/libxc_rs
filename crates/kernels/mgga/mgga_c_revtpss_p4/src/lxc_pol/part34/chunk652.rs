//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 652/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk652<F: Float>(t482: F, t6628: F, t3604: F, t1042: F, t3611: F, t1469: F, t3628: F, t5351: F, t3626: F, t6587: F, t371: F, t372: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6629 = t482 * t6628;
    let t6630 = t6629 * t3604;
    let t6631 = t1042 * t6630;
    let t6634 = t6629 * t3611;
    let t6635 = t1042 * t6634;
    let t6638 = t3628 * t1469;
    let t6639 = t5351 * t6638;
    let t6640 = t3626 * t6639;
    let t6645 = t482 * t6587;
    let t6647 = t371 * t372 * t6645;
    (t6630, t6631, t6634, t6635, t6638, t6639, t6640, t6645, t6647)
}
