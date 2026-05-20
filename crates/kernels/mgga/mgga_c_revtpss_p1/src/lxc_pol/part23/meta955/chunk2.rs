//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3184/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3184<F: Float>(t12832: F, t17351: F, t17401: F, t17654: F, t20929: F, t20978: F, t21042: F, t24706: F, t3604: F, t3611: F, t44607: F, t5047: F, t5052: F, t56879: F, t57615: F, t57636: F, t57663: F, t57687: F, t57707: F, t6638: F, t69839: F, t70794: F, t83125: F) -> F {
    let t83640 = F::cast_from(0.19055119163586549765e-3_f64) * t57615 - t44607 - t57636 - F::cast_from(0.42874018118069736972e-3_f64) * t56879 * t69839 * t70794 * t6638 + F::cast_from(0.85748036236139473944e-3_f64) * t17351 * t69839 * t3611 * t5052 + F::cast_from(0.14291339372689912324e-2_f64) * t17654 * t83125 * t3604 * t5047 + F::cast_from(0.85748036236139473947e-3_f64) * t57663 * t20929 - F::new(5.0) / F::new(1296.0) * t57687 - F::cast_from(0.64311027177104605458e-3_f64) * t17401 * t21042 + F::cast_from(0.68598428988911579154e-2_f64) * t57707 * t20978 - F::cast_from(0.64311027177104605458e-3_f64) * t12832 * t24706;
    t83640
}
