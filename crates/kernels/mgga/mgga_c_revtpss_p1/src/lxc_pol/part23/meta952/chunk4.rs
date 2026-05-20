//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3159/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3159<F: Float>(t1222: F, t140: F, t24816: F, t24820: F, t12915: F, t247: F, t24713: F, t5384: F, t12866: F, t13046: F, t13053: F, t17307: F, t17654: F, t20765: F, t20932: F, t20933: F, t20941: F, t3604: F, t3611: F, t44517: F, t5052: F, t5386: F, t5390: F, t5401: F, t56997: F, t57663: F, t59066: F, t69839: F, t71112: F, t71300: F) -> F {
    let t82980 = t1222 * t140 * t24816;
    let t82983 = t1222 * t140 * t24820;
    let t83014 = t5384 * t247 * t12915 * t24713;
    let t83016 = -t82980 / F::new(864.0) - t82983 / F::new(144.0) - F::cast_from(0.25724410870841842183e-2_f64) * t56997 * t69839 * t13046 * t20765 + F::cast_from(0.25724410870841842183e-2_f64) * t59066 * t69839 * t13053 * t20765 - F::cast_from(0.17149607247227894789e-2_f64) * t17654 * t69839 * t3604 * t5052 - F::cast_from(0.42874018118069736972e-3_f64) * t44517 * t69839 * t3611 * t20932 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t71112 * t5401 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t71300 * t20933 + F::cast_from(0.85748036236139473944e-3_f64) * t57663 * t20941 - F::cast_from(0.13719685797782315831e-1_f64) * t17307 * t5390 * t5386 + F::cast_from(0.85748036236139473947e-3_f64) * t83014;
    t83016
}
