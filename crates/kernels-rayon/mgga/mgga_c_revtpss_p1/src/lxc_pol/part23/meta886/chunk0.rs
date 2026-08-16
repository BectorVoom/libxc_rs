//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2800/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2800(t2289: f64, t5892: f64, t21821: f64, t625: f64, t21824: f64, t1455: f64, t6951: f64, t1464: f64, t6936: f64, t22571: f64, t571: f64, t25048: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75639 = t2289 * t5892;
    let t75641 = t625 * t21821;
    let t75643 = t625 * t21824;
    let t75720 = t1455 * t6951;
    let t75727 = t6936 * t1464;
    let t75796 = t571 * t22571;
    let t75808 = t25048 * t575;
    (t75639, t75641, t75643, t75720, t75727, t75796, t75808)
}
