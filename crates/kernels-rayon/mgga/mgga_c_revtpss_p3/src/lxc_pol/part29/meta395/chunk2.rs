//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1416/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1416(t1548: f64, t2394: f64, t800: f64, t2689: f64, t4372: f64, t4354: f64, t9775: f64, t14468: f64, t828: f64, t855: f64, t221: f64, t2675: f64, t4343: f64) -> (f64, f64, f64, f64, f64) {
    let t14843 = t800 * t1548 * t2394;
    let t14846 = t2689 * t4372;
    let t14850 = t9775 * t4354;
    let t14853 = t855 * t828 * t14468;
    let t14857 = t2675 * t221 * t4343;
    (t14843, t14846, t14850, t14853, t14857)
}
