//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 645/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk645(t205: f64, t626: f64, t191: f64, t1641: f64, t261: f64, t4367: f64, t1691: f64, t5024: f64, t11: f64, t5029: f64, t5033: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5060 = 1.0_f64 / t205 / t626;
    let t5061 = t191 * t5060;
    let t5063 = 1.0_f64 / t1641 / t261;
    let t5064 = t5063 * t4367;
    let t5065 = t5061 * t5064;
    let t5068 = t1691 * t5024;
    let t5069 = t11 * t5068;
    let t5071 = t1691 * t5029;
    let t5072 = t11 * t5071;
    let t5074 = t625 * t5033;
    (t5061, t5063, t5064, t5065, t5068, t5069, t5071, t5072, t5074)
}
