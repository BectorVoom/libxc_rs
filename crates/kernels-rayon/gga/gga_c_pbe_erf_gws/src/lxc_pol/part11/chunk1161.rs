//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1161/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1161(t43: f64, t21975: f64, t30116: f64, t22063: f64, t42442: f64, t22066: f64, t22068: f64, t22070: f64, t12345: f64, t1402: f64, t18670: f64, t2457: f64, t3346: f64, t47: f64, t47391: f64, t47400: f64, t47409: f64, t9981: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t48440 = 384.0_f64 * t21975;
    let t48441 = 6.0_f64 * t30116;
    let t48442 = 240.0_f64 * t22063;
    let t48443 = 0.73246220147012639764e-3_f64 * t42442;
    let t48444 = 0.65061485296689145286e-1_f64 * t22066;
    let t48445 = 0.1926377843805564792e1_f64 * t22068;
    let t48446 = 0.86748647062252193714e-1_f64 * t22070;
    let t48458 = piecewise3(t44, 0.0_f64, 40.0_f64 / 81.0_f64 * t18670 * t47391 - 16.0_f64 / 9.0_f64 * t9981 * t3346 + 4.0_f64 / 3.0_f64 * t1402 * t47409 + 16.0_f64 / 9.0_f64 * t2457 * t12345 + 4.0_f64 / 3.0_f64 * t47 * t47400);
    (t48440, t48441, t48442, t48443, t48444, t48445, t48446, t48458)
}
