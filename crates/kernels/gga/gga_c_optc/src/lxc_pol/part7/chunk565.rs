//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 565/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk565<F: Float>(t2724: F, t2813: F, t2591: F, t2598: F, t2758: F, t2761: F, t2766: F, t2773: F, t2775: F, t2778: F, t2781: F, t2786: F, t2790: F, t2797: F, t2801: F, t2803: F, t2806: F, t2809: F, t2812: F, t913: F, t930: F, t931: F, t940: F, t953: F) -> (F, F) {
    let t2814 = t2813 * t2724;
    let t2817 = -F::cast_from(0.77431140607485233683e1_f64) * t2758 * t2761 + F::cast_from(0.5848048239485271795e1_f64) * t940 * t2766 + F::cast_from(0.8790987341241436962e3_f64) * t2773 * t2775 - F::cast_from(0.4395493670620718481e3_f64) * t2778 * t2781 + F::cast_from(0.11360101276506094136e1_f64) * t913 * t2786 + F::cast_from(0.779739765264702906e1_f64) * t2790 + F::cast_from(0.50380704458364197288e-2_f64) * t953 * t2591 + F::cast_from(0.83967840763940328814e-2_f64) * t953 * t2598 - F::cast_from(0.15454509315180013964e0_f64) * t2797 * t931 + F::cast_from(0.19318136643975017455e-1_f64) * t2801 + F::cast_from(0.28977204965962526182e-1_f64) * t930 * t2803 + F::cast_from(0.38636273287950034909e-1_f64) * t930 * t2806 + F::cast_from(0.6717427261115226305e-2_f64) * t2809 + F::cast_from(0.779739765264702906e1_f64) * t2812 * t2814;
    (t2814, t2817)
}
