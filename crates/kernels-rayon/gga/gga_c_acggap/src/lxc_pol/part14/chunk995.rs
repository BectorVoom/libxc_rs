//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 995/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk995(t35210: f64, t30219: f64, t8515: f64, t1345: f64, t1992: f64, t30154: f64, t7586: f64, t1535: f64, t4180: f64, t7646: f64, t1181: f64, t30327: f64, t4358: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35211 = 0.94344276868812456204e-2_f64 * t35210;
    let t35212 = t30219 * t8515;
    let t35213 = 0.21437009059034868486e-2_f64 * t35212;
    let t35225 = t1992 * t1345;
    let t35227 = t30154 * t7586 * t35225;
    let t35228 = 0.14291339372689912324e-2_f64 * t35227;
    let t35230 = t4180 * t7646 * t1535;
    let t35231 = 0.17149607247227894789e-2_f64 * t35230;
    let t35238 = t30327 * t1181 * t599 * t4358;
    (t35211, t35213, t35225, t35228, t35231, t35238)
}
