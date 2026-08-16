//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 894/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk894(t16954: f64, t5309: f64, t7136: f64, t1898: f64, t5304: f64, t2704: f64, t628: f64, t1243: f64, t1703: f64, t1693: f64, t395: f64, t5093: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16955 = 64.0_f64 / 45.0_f64 * t16954;
    let t16957 = 16.0_f64 / 5.0_f64 * t7136 * t5309;
    let t16959 = 32.0_f64 / 15.0_f64 * t5304 * t1898;
    let t16960 = t2704 * t628;
    let t16962 = t1243 * t1703;
    let t16964 = t1243 * t1693;
    let t16966 = t395 * t5093;
    (t16955, t16957, t16959, t16960, t16962, t16964, t16966)
}
