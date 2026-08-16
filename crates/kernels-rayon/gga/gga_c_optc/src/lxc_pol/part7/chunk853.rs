//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 853/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk853(t2724: f64, t8152: f64, t2721: f64, t2633: f64, t7178: f64, t914: f64, t2596: f64, t894: f64, t2264: f64, t2723: f64, t3836: f64, t2722: f64, t8044: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8153 = t8152 * t2724;
    let t8154 = t2721 * t8153;
    let t8156 = t2633 * t7178;
    let t8157 = t914 * t8156;
    let t8160 = t2596 * t7178;
    let t8161 = t894 * t8160;
    let t8164 = t2723 * t2264;
    let t8165 = t3836 * t8164;
    let t8168 = t2722 * t8044;
    (t8153, t8154, t8156, t8157, t8160, t8161, t8164, t8165, t8168)
}
