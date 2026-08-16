//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2856/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2856(t50084: f64, t61239: f64, t50092: f64, t50094: f64, t23221: f64, t2398: f64, t61247: f64, t61282: f64, t61289: f64, t50852: f64, t50856: f64, t61294: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77002 = 12.0_f64 * t50084;
    let t77003 = 0.17544670867903938621e1_f64 * t61239;
    let t77004 = 0.48796115851357829289e-1_f64 * t50092;
    let t77005 = 0.14447919941302971323e1_f64 * t50094;
    let t77007 = 4.0_f64 * t2398 * t23221;
    let t77008 = 0.32530743900905219526e-1_f64 * t61247;
    let t77009 = 0.73245789224026180216e-3_f64 * t61282;
    let t77010 = 24.0_f64 * t61289;
    let t77011 = 0.15584273195113317383e3_f64 * t50852;
    let t77012 = 0.17090684152272775384e-2_f64 * t50856;
    let t77013 = 0.17544670867903938621e1_f64 * t61294;
    (t77002, t77003, t77004, t77005, t77007, t77008, t77009, t77010, t77011, t77012, t77013)
}
