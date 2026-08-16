//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 639/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk639(t2030: f64, t549: f64, t554: f64, t2057: f64, t538: f64, t138: f64, t8153: f64, t8157: f64, t550: f64, t2044: f64, t7853: f64, t131: f64, t1991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8865 = t549 * t2030;
    let t8866 = t8865 * t554;
    let t8869 = t2057 * t538;
    let t8873 = t138 * t8153;
    let t8874 = t8873 * t8157;
    let t8877 = t2057 * t554;
    let t8883 = t550 * t538;
    let t8885 = t7853 * t2044;
    let t8894 = t1991 * t131;
    (t8865, t8866, t8869, t8873, t8874, t8877, t8883, t8885, t8894)
}
