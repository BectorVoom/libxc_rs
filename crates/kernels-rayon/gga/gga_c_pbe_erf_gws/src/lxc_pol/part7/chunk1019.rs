//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1019/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1019(t119: f64, t1477: f64, t391: f64, t11: f64, t174: f64, t2: f64, t6045: f64, t413: f64, t4528: f64, t1246: f64, t18490: f64, t398: f64) -> (f64, f64, f64, f64, f64) {
    let t18493 = t119 * t1477;
    let t18494 = t391 * t18493;
    let t18497 = f64::powf(t11, -0.25e1_f64);
    let t18500 = t18497 * t2 * t6045 * t174;
    let t18502 = t4528 * t413;
    let t18504 = t1246 * t18490;
    let t18506 = t398 * t18493;
    (t18494, t18500, t18502, t18504, t18506)
}
