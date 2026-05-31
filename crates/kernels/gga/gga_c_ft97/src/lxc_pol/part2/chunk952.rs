//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 952/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk952<F: Float>(t1111: F, t1472: F, t14721: F, t14725: F, t14729: F, t14731: F, t14734: F, t14739: F, t14742: F, t14746: F, t14749: F, t14753: F, t14756: F, t14760: F, t14763: F, t14766: F, t2691: F, t2726: F, t4065: F, t4113: F) -> F {
    let t14769 = F::cast_from(0.2416365355361531912e1_f64) * t14721 * t14725 + F::cast_from(0.1208182677680765956e1_f64) * t14729 * t14731 - F::cast_from(6.0_f64) * t4113 * t14734 * t2726 + F::cast_from(4.0_f64) * t2691 * t14739 - F::cast_from(0.1208182677680765956e1_f64) * t14742 * t14731 - F::cast_from(0.60409133884038297798e0_f64) * t1472 * t14746 - F::cast_from(2.0_f64) * t2691 * t14749 - F::cast_from(4.0_f64) * t2691 * t14753 - F::cast_from(2.0_f64) * t2691 * t14756 - F::cast_from(0.1208182677680765956e1_f64) * t14760 * t1111 - F::cast_from(4.0_f64) * t14763 * t4065 - F::cast_from(0.2416365355361531912e1_f64) * t14766 * t14725;
    t14769
}
