//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 763/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk763(t12484: f64, t173: f64, t184: f64, t199: f64, t12350: f64, t5063: f64, t5089: f64, t11: f64, t5002: f64, t1691: f64, t2678: f64, t3354: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12485 = t173 * t12484;
    let t12486 = t12485 * t184;
    let t12488 = 2.0_f64 / 15.0_f64 * t12486 * t199;
    let t12493 = t5063 * t12350;
    let t12494 = t5089 * t12493;
    let t12495 = t11 * t12494;
    let t12497 = t5002 * t12350;
    let t12498 = t1691 * t12497;
    let t12499 = t11 * t12498;
    let t12501 = t2678 * t3354;
    (t12485, t12486, t12488, t12493, t12494, t12495, t12497, t12498, t12499, t12501)
}
