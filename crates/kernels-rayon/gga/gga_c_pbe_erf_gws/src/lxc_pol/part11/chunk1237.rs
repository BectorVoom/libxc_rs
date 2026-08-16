//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1237/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1237(t45128: f64, t9016: f64, t13525: f64, t37768: f64, t2157: f64, t3703: f64, t11478: f64, t3138: f64, t6523: f64, t13220: f64, t20842: f64, t2168: f64, t3131: f64, t343: f64) -> (f64, f64, f64, f64, f64) {
    let t49561 = t9016 * t45128 / 4.0_f64;
    let t49567 = t37768 * t13525 / 12.0_f64;
    let t49568 = t2157 * t3703;
    let t49572 = 3.0_f64 / 4.0_f64 * t3138 * t6523 * t11478 * t49568;
    let t49576 = t2168 * t20842 * t3131 * t343 * t13220;
    (t49561, t49567, t49568, t49572, t49576)
}
