//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 511/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk511(t241: f64, t2690: f64, t244: f64, t248: f64, t238: f64, t835: f64, t841: f64, t812: f64, t849: f64, t1891: f64, t67: f64, t2379: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2691 = t2690 * t241;
    let t2693 = t2691 * t244 * t248;
    let t2695 = 119.0_f64 / 13824.0_f64 * t238 * t2693;
    let t2696 = t841 * t835;
    let t2697 = t812 * t2696;
    let t2698 = t2697 * t849;
    let t2700 = t241 * t1891;
    let t2701 = t2700 * t67;
    let t2703 = t2701 * t820 * t2379;
    (t2691, t2693, t2695, t2696, t2697, t2698, t2701, t2703)
}
