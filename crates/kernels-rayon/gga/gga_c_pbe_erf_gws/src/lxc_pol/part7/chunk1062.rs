//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1062/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1062(t147: f64, t159: f64, t285: f64, t4259: f64, t169: f64, t301: f64, t745: f64, t922: f64, t5631: f64, t755: f64, t759: f64, t1452: f64, t366: f64) -> (f64, f64, f64, f64, f64) {
    let t19174 = 0.10943113336969376162e-5_f64 * t4259 * t147 * t159 * t285;
    let t19177 = t169 * t922 * t745 * t301;
    let t19179 = t5631 * t755;
    let t19182 = 0.78054266140918933351e0_f64 * t5631 * t759;
    let t19185 = t169 * t366 * t1452 * t301;
    (t19174, t19177, t19179, t19182, t19185)
}
