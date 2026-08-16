//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3159/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3159(t1222: f64, t140: f64, t24816: f64, t24820: f64, t12915: f64, t247: f64, t24713: f64, t5384: f64, t12866: f64, t13046: f64, t13053: f64, t17307: f64, t17654: f64, t20765: f64, t20932: f64, t20933: f64, t20941: f64, t3604: f64, t3611: f64, t44517: f64, t5052: f64, t5386: f64, t5390: f64, t5401: f64, t56997: f64, t57663: f64, t59066: f64, t69839: f64, t71112: f64, t71300: f64) -> f64 {
    let t82980 = t1222 * t140 * t24816;
    let t82983 = t1222 * t140 * t24820;
    let t83014 = t5384 * t247 * t12915 * t24713;
    let t83016 = -t82980 / 864.0_f64 - t82983 / 144.0_f64 - 0.25724410870841842183e-2_f64 * t56997 * t69839 * t13046 * t20765 + 0.25724410870841842183e-2_f64 * t59066 * t69839 * t13053 * t20765 - 0.17149607247227894789e-2_f64 * t17654 * t69839 * t3604 * t5052 - 0.42874018118069736972e-3_f64 * t44517 * t69839 * t3611 * t20932 + 0.42874018118069736972e-3_f64 * t12866 * t71112 * t5401 + 0.42874018118069736972e-3_f64 * t12866 * t71300 * t20933 + 0.85748036236139473944e-3_f64 * t57663 * t20941 - 0.13719685797782315831e-1_f64 * t17307 * t5390 * t5386 + 0.85748036236139473947e-3_f64 * t83014;
    t83016
}
