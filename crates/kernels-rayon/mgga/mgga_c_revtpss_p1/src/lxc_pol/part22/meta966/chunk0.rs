//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3229/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3229(t4343: f64, t177: f64, t18550: f64, t762: f64, t50092: f64, t50094: f64, t123: f64, t2630: f64, t5941: f64, t50097: f64, t50099: f64, t14390: f64, t18259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t61234 = t4343 * t4343;
    let t61239 = t18550 * t177 * t762;
    let t61240 = 0.11696447245269292414e1_f64 * t61239;
    let t61244 = 0.32530743900905219526e-1_f64 * t50092;
    let t61245 = 0.96319466275353142155e0_f64 * t50094;
    let t61247 = t5941 * t123 * t2630;
    let t61248 = 0.10843581300301739842e-1_f64 * t61247;
    let t61249 = 4.0_f64 * t50097;
    let t61250 = 16.0_f64 * t50099;
    let t61261 = 48.0_f64 * t18259 * t14390;
    (t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261)
}
