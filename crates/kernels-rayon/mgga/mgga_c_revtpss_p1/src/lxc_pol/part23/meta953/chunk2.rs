//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3165/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3165(t12855: f64, t12916: f64, t24835: f64, t17729: f64, t20317: f64, t20802: f64, t20806: f64, t20952: f64, t20978: f64, t21049: f64, t21306: f64, t24734: f64, t3626: f64, t3720: f64, t5331: f64, t59162: f64, t70112: f64, t70114: f64, t70129: f64, t70133: f64, t70311: f64, t82481: f64) -> f64 {
    let t83158 = t12855 * t12916 * t24835;
    let t83170 = -0.64311027177104605458e-3_f64 * t5331 * t3720 * t70311 * t24734 + 0.12862205435420921092e-2_f64 * t21049 * t20802 - 0.64311027177104605458e-3_f64 * t21306 * t20806 - 0.12862205435420921092e-2_f64 * t59162 * t20978 - 0.85748036236139473947e-3_f64 * t83158 + 0.25724410870841842184e-2_f64 * t21049 * t20952 + 0.85748036236139473944e-3_f64 * t17729 * t3626 * t20317 * t82481 - 0.10162730220579493208e-2_f64 * t70112 - 0.96545937095505185473e-2_f64 * t70114 - 0.25724410870841842183e-2_f64 * t70129 - 0.15879265969655458138e-3_f64 * t70133;
    t83170
}
