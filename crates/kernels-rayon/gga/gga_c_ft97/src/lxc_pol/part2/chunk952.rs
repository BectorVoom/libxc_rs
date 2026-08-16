//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 952/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk952(t1111: f64, t1472: f64, t14721: f64, t14725: f64, t14729: f64, t14731: f64, t14734: f64, t14739: f64, t14742: f64, t14746: f64, t14749: f64, t14753: f64, t14756: f64, t14760: f64, t14763: f64, t14766: f64, t2691: f64, t2726: f64, t4065: f64, t4113: f64) -> f64 {
    let t14769 = 0.2416365355361531912e1_f64 * t14721 * t14725 + 0.1208182677680765956e1_f64 * t14729 * t14731 - 6.0_f64 * t4113 * t14734 * t2726 + 4.0_f64 * t2691 * t14739 - 0.1208182677680765956e1_f64 * t14742 * t14731 - 0.60409133884038297798e0_f64 * t1472 * t14746 - 2.0_f64 * t2691 * t14749 - 4.0_f64 * t2691 * t14753 - 2.0_f64 * t2691 * t14756 - 0.1208182677680765956e1_f64 * t14760 * t1111 - 4.0_f64 * t14763 * t4065 - 0.2416365355361531912e1_f64 * t14766 * t14725;
    t14769
}
