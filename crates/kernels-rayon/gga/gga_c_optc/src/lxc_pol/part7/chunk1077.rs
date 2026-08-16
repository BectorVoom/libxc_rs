//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1077/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1077(t1974: f64, t1956: f64, t2229: f64, t104: f64, t1879: f64, t1948: f64, t22340: f64, t22342: f64, t22344: f64, t22621: f64, t22623: f64, t22625: f64, t22627: f64, t23390: f64, t6312: f64, t6856: f64, t712: f64, t714: f64, t95: f64) -> f64 {
    let t23392 = t1974 * t1974;
    let t23393 = 1.0_f64 / t23392;
    let t23400 = t2229 * t1956;
    let t23402 = t22340 + 0.62027715443768233192e-1_f64 * t95 * t6856 * t712 * t714 + t22342 - 0.15506928860942058298e-1_f64 * t95 * t104 * t23390 * t23393 + t22344 + t22621 - t22623 + t22625 - t22627 + 0.46520786582826174894e-1_f64 * t1879 * t6312 * t1948 + 3.0_f64 * t23400;
    t23402
}
