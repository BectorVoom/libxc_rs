//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 943/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk943(t17541: f64, t564: f64, t2730: f64, t5171: f64, t1365: f64, t670: f64, t671: f64, t1985: f64, t666: f64, t226: f64, t5903: f64, t230: f64, t5907: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17543 = 16.0_f64 / 15.0_f64 * t17541 * t564;
    let t17544 = t2730 * t5171;
    let t17545 = 32.0_f64 / 15.0_f64 * t17544;
    let t17548 = 0.22443641344164119597e0_f64 * t670 * t1365 * t671;
    let t17549 = t666 * t1985;
    let t17552 = 16.0_f64 / 3.0_f64 * t226 * t5903;
    let t17553 = t5907 * t230;
    (t17543, t17545, t17548, t17549, t17552, t17553)
}
