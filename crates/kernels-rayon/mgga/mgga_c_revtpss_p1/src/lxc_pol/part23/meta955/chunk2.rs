//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3184/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3184(t12832: f64, t17351: f64, t17401: f64, t17654: f64, t20929: f64, t20978: f64, t21042: f64, t24706: f64, t3604: f64, t3611: f64, t44607: f64, t5047: f64, t5052: f64, t56879: f64, t57615: f64, t57636: f64, t57663: f64, t57687: f64, t57707: f64, t6638: f64, t69839: f64, t70794: f64, t83125: f64) -> f64 {
    let t83640 = 0.19055119163586549765e-3_f64 * t57615 - t44607 - t57636 - 0.42874018118069736972e-3_f64 * t56879 * t69839 * t70794 * t6638 + 0.85748036236139473944e-3_f64 * t17351 * t69839 * t3611 * t5052 + 0.14291339372689912324e-2_f64 * t17654 * t83125 * t3604 * t5047 + 0.85748036236139473947e-3_f64 * t57663 * t20929 - 5.0_f64 / 1296.0_f64 * t57687 - 0.64311027177104605458e-3_f64 * t17401 * t21042 + 0.68598428988911579154e-2_f64 * t57707 * t20978 - 0.64311027177104605458e-3_f64 * t12832 * t24706;
    t83640
}
