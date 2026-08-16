//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3165/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3165<F: Float>(t12855: F, t12916: F, t24835: F, t17729: F, t20317: F, t20802: F, t20806: F, t20952: F, t20978: F, t21049: F, t21306: F, t24734: F, t3626: F, t3720: F, t5331: F, t59162: F, t70112: F, t70114: F, t70129: F, t70133: F, t70311: F, t82481: F) -> F {
    let t83158 = t12855 * t12916 * t24835;
    let t83170 = -F::cast_from(0.64311027177104605458e-3_f64) * t5331 * t3720 * t70311 * t24734 + F::cast_from(0.12862205435420921092e-2_f64) * t21049 * t20802 - F::cast_from(0.64311027177104605458e-3_f64) * t21306 * t20806 - F::cast_from(0.12862205435420921092e-2_f64) * t59162 * t20978 - F::cast_from(0.85748036236139473947e-3_f64) * t83158 + F::cast_from(0.25724410870841842184e-2_f64) * t21049 * t20952 + F::cast_from(0.85748036236139473944e-3_f64) * t17729 * t3626 * t20317 * t82481 - F::cast_from(0.10162730220579493208e-2_f64) * t70112 - F::cast_from(0.96545937095505185473e-2_f64) * t70114 - F::cast_from(0.25724410870841842183e-2_f64) * t70129 - F::cast_from(0.15879265969655458138e-3_f64) * t70133;
    t83170
}
