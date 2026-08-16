//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 625/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk625(t1232: f64, t1238: f64, t142: f64, t1246: f64, t458: f64, t462: f64, t153: f64, t1564: f64, t169: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t4066 = t1232 * t1232;
    let t4072 = t1238 * t1238;
    let t4074 = 1.0_f64 / t4072 / t142;
    let t4077 = pi * t1246 * t458;
    let t4080 = t462 * t462;
    let t4081 = 1.0_f64 / t4080;
    let t4082 = t153 * t4081;
    let t4085 = t4074 * pi * t458;
    let t4130 = t169 * t1564;
    (t4066, t4072, t4074, t4077, t4080, t4081, t4082, t4085, t4130)
}
