//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 942/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk942(t1: f64, t6382: f64, t253: f64, t2118: f64, t8986: f64, t6201: f64, t916: f64, t2250: f64, t2132: f64, t6472: f64, t20133: f64, t326: f64) -> (f64, f64, f64, f64, f64) {
    let t21518 = t6382 * t1;
    let t21519 = t21518 * t253;
    let t21529 = t2118 * t8986;
    let t21535 = t6201 * t916;
    let t21536 = t2250 * t21535;
    let t21597 = t6472 * t2132;
    let t21621 = t326 * t20133;
    (t21519, t21529, t21536, t21597, t21621)
}
