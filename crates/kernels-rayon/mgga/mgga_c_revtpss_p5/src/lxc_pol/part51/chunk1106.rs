//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1106/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1106(t120952: f64, t1885: f64, t32284: f64, t5661: f64, t121107: f64, t5665: f64, t121110: f64, t1444: f64, t1868: f64, t120956: f64, t1414: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125578 = t120952 * t1885;
    let t125580 = t32284 * t5661;
    let t125582 = t121107 * t5665;
    let t125584 = t121110 * t5665;
    let t125587 = t1868 * t1444;
    let t125590 = t120956 * t1414 * t828 * t125587;
    (t125578, t125580, t125582, t125584, t125587, t125590)
}
