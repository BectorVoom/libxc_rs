//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 565/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk565(t2724: f64, t2813: f64, t2591: f64, t2598: f64, t2758: f64, t2761: f64, t2766: f64, t2773: f64, t2775: f64, t2778: f64, t2781: f64, t2786: f64, t2790: f64, t2797: f64, t2801: f64, t2803: f64, t2806: f64, t2809: f64, t2812: f64, t913: f64, t930: f64, t931: f64, t940: f64, t953: f64) -> (f64, f64) {
    let t2814 = t2813 * t2724;
    let t2817 = -0.77431140607485233683e1_f64 * t2758 * t2761 + 0.5848048239485271795e1_f64 * t940 * t2766 + 0.8790987341241436962e3_f64 * t2773 * t2775 - 0.4395493670620718481e3_f64 * t2778 * t2781 + 0.11360101276506094136e1_f64 * t913 * t2786 + 0.779739765264702906e1_f64 * t2790 + 0.50380704458364197288e-2_f64 * t953 * t2591 + 0.83967840763940328814e-2_f64 * t953 * t2598 - 0.15454509315180013964e0_f64 * t2797 * t931 + 0.19318136643975017455e-1_f64 * t2801 + 0.28977204965962526182e-1_f64 * t930 * t2803 + 0.38636273287950034909e-1_f64 * t930 * t2806 + 0.6717427261115226305e-2_f64 * t2809 + 0.779739765264702906e1_f64 * t2812 * t2814;
    (t2814, t2817)
}
