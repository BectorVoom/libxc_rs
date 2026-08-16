//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 977/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk977(t1: f64, t17855: f64, t438: f64, t450: f64, t16236: f64, t8516: f64, t894: f64, t17352: f64, t3235: f64, t17344: f64, t4387: f64, t1121: f64, t1133: f64, t12098: f64, t12106: f64, t17700: f64, t17705: f64, t17710: f64, t17714: f64, t17720: f64, t17724: f64, t17729: f64, t3132: f64, t4369: f64, t4386: f64, t5298: f64, t5302: f64, t8913: f64, t8921: f64, t8960: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17857 = t17855 * t1 * t438;
    let t17858 = t450 * t17857;
    let t17863 = t8516 * t16236;
    let t17864 = t894 * t17863;
    let t17869 = t3235 * t17352;
    let t17872 = t4387 * t17344;
    let t17875 = 0.17715845405452227366e4_f64 * t8960 * t17700 + 0.10629507243271336419e5_f64 * t8913 * t17705 - 0.10629507243271336419e5_f64 * t8921 * t17710 + 0.10866451862235947318e-1_f64 * t1133 * t17714 - 0.48295341609937543638e-1_f64 * t4369 * t5302 + 0.18110753103726578864e-2_f64 * t1133 * t17720 + 0.80492236016562572728e-2_f64 * t1133 * t17724 - 0.13735917720689745254e2_f64 * t3132 * t17729 + 0.35500316489081544176e-1_f64 * t1121 * t17858 - 0.28977204965962526182e-1_f64 * t4369 * t5298 - 0.18110753103726578864e-1_f64 * t1133 * t17864 + 0.96590683219875087275e-2_f64 * t12098 - 0.23666877659387696117e-1_f64 * t12106 - 0.10866451862235947318e-1_f64 * t4386 * t17869 + 0.90553765518632894319e-2_f64 * t4386 * t17872;
    (t17857, t17858, t17863, t17864, t17869, t17872, t17875)
}
