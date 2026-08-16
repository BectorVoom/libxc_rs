//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 743/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk743(t172: f64, t1948: f64, t201: f64, t755: f64, t1953: f64, t3318: f64, t104: f64, t1879: f64, t1880: f64, t1928: f64, t3316: f64, t3539: f64, t606: f64, t616: f64, t6477: f64, t6560: f64, t6811: f64, t6816: f64, t6819: f64, t6856: f64, t714: f64, t7142: f64, t95: f64) -> (f64, f64) {
    let t7153 = t172 * t1948;
    let t7157 = t755 * t201;
    let t7158 = t7157 * t1953;
    let t7159 = t3318 * t7158;
    let t7168 = 0.25844881434903430496e-2_f64 * t95 * t104 * t7142 * t714 + 0.77534644304710291488e-2_f64 * t95 * t606 * t6560 + 0.23260393291413087447e-1_f64 * t1879 * t1880 * t1948 + 0.46520786582826174894e-1_f64 * t3539 * t7153 * t616 + 3.0_f64 / 2.0_f64 * t3316 * t7159 + t6811 + 0.15506928860942058298e-1_f64 * t95 * t6856 * t172 + t6477 + t6816 + 0.46520786582826174894e-1_f64 * t3539 * t1880 * t1928 - t6819;
    (t7159, t7168)
}
