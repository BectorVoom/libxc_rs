//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1116/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1116(t27375: f64, t27383: f64, t198: f64, t8539: f64, t27384: f64, t98785: f64, t1544: f64, t7086: f64, t25207: f64, t18875: f64, t2411: f64, t33726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t125977 = t27383 * t27375;
    let t125980 = t198 * t8539;
    let t125981 = t98785 * t27384;
    let t125984 = t1544 * t7086;
    let t125985 = t25207 * t125984;
    let t125988 = t27383 * t18875;
    let t125997 = t33726 * t2411;
    (t125977, t125980, t125981, t125984, t125985, t125988, t125997)
}
