//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 996/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk996(t243: f64, t247: f64, t9949: f64, t237: f64, t236: f64, t9646: f64, t9721: f64, t268: f64, t207: f64, t242: f64, t240: f64, t72: f64) -> (f64, f64, f64) {
    let t10685 = t9949 * t243 * t247;
    let t10687 = 0.37792653007779990369e-1_f64 * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = 0.20082057720118594944e-6_f64 * t10688 * t10690;
    let t10696 = 1.0_f64 / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    (t10687, t10692, t10698)
}
