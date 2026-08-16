//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1318/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1318(t246: f64, t9645: f64, t232: f64, t2379: f64, t2628: f64, t835: f64, t812: f64, t2635: f64, t2690: f64, t815: f64, t831: f64, t2617: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9668 = t9667 * t2635;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    let t9674 = t2617 * t2638;
    (t9646, t9647, t9668, t9671, t9672, t9674)
}
