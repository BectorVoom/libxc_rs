//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 908/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk908(t482: f64, t6628: f64, t3604: f64, t1042: f64, t3611: f64, t1469: f64, t3628: f64, t5351: f64, t3626: f64, t6587: f64, t371: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
