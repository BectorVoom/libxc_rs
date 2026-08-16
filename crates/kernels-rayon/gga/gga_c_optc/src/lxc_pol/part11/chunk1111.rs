//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1111/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1111(t140: f64, t2665: f64, t5255: f64, t3183: f64, t3101: f64, t12489: f64, t4444: f64, t12726: f64, t4450: f64, t3201: f64, t5421: f64, t1135: f64, t5311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45769 = t5255 * t2665 * t140;
    let t45770 = t3183 * t45769;
    let t45773 = t3101 * t45769;
    let t45788 = t4444 * t12489;
    let t45795 = t4450 * t12726;
    let t45809 = t5421 * t3201;
    let t45811 = t1135 * t5311;
    (t45770, t45773, t45788, t45795, t45809, t45811)
}
