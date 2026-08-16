//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 793/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk793(t1479: f64, t553: f64, t535: f64, t837: f64, t551: f64, t1371: f64, t1952: f64, t1378: f64, t1971: f64, t5697: f64, t1354: f64, t331: f64) -> (f64, f64, f64, f64, f64) {
    let t6005 = 0.258995450979035416e-1_f64 * t1479 * t553;
    let t6006 = t837 * t535;
    let t6008 = t6006 * t551 * t553;
    let t6012 = 0.19753890328909480882e-1_f64 * t1952 * t1371 * t553;
    let t6015 = 0.34679929861433484636e-2_f64 * t5697 * t1378 * t1971;
    let t6016 = t331 * t1354;
    (t6005, t6008, t6012, t6015, t6016)
}
