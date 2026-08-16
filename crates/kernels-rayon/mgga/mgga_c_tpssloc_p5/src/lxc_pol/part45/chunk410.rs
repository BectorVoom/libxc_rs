//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 410/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk410(t2684: f64, t819: f64, t820: f64, t20: f64, t61: f64, t241: f64, t244: f64, t248: f64, t238: f64, t835: f64, t841: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2686 = t819 * t820 * t2684;
    let t2690 = 1.0_f64 / t61 / t20;
    let t2691 = t2690 * t241;
    let t2693 = t2691 * t244 * t248;
    let t2695 = 119.0_f64 / 13824.0_f64 * t238 * t2693;
    let t2696 = t841 * t835;
    let t2697 = t812 * t2696;
    (t2686, t2690, t2691, t2693, t2695, t2697)
}
