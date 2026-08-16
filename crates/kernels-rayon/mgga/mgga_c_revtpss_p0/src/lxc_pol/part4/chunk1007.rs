//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1007/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1007(t10674: f64, t236: f64, t807: f64, t2689: f64, t2694: f64, t2430: f64, t854: f64, t243: f64, t247: f64, t9949: f64, t237: f64, t9646: f64) -> (f64, f64, f64, f64, f64) {
    let t10675 = t236 * t10674;
    let t10676 = t807 * t10675;
    let t10678 = t2689 * t2694;
    let t10680 = t854 * t2430;
    let t10681 = t236 * t10680;
    let t10682 = t807 * t10681;
    let t10685 = t9949 * t243 * t247;
    let t10687 = 0.37792653007779990369e-1_f64 * t237 * t10685;
    let t10688 = t9646 * t236;
    (t10676, t10678, t10682, t10687, t10688)
}
