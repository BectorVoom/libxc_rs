//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 610/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk610(t2722: f64, t5025: f64, t4937: f64, t914: f64, t4929: f64, t4933: f64, t2813: f64, t2721: f64, t2729: f64, t2745: f64, t2751: f64, t2758: f64, t2773: f64, t2778: f64, t2812: f64, t3892: f64, t3897: f64, t3947: f64, t3952: f64, t4972: f64, t4976: f64, t4980: f64, t4998: f64, t5003: f64, t5008: f64, t5012: f64, t5017: f64, t5022: f64, t913: f64, t930: f64, t940: f64, t953: f64) -> f64 {
    let t5026 = t2722 * t5025;
    let t5037 = t914 * t4937;
    let t5040 = t914 * t4929;
    let t5043 = t914 * t4933;
    let t5046 = t2813 * t5025;
    let t5049 = 0.779739765264702906e1_f64 * t3947 + 0.75734008510040627574e0_f64 * t3952 + 0.23229342182245570105e2_f64 * t2751 * t4998 - 0.77431140607485233683e1_f64 * t2758 * t5003 + 0.5848048239485271795e1_f64 * t940 * t5008 + 0.8790987341241436962e3_f64 * t2773 * t5012 - 0.4395493670620718481e3_f64 * t2778 * t5017 + 0.11360101276506094136e1_f64 * t913 * t5022 - t2729 - t2745 + 0.75734008510040627574e0_f64 * t2721 * t5026 + 0.6717427261115226305e-2_f64 * t3892 + 0.19318136643975017455e-1_f64 * t3897 - 0.10076140891672839458e-1_f64 * t953 * t4976 + 0.50380704458364197288e-2_f64 * t953 * t4980 + 0.83967840763940328814e-2_f64 * t953 * t4972 + 0.28977204965962526182e-1_f64 * t930 * t5037 + 0.38636273287950034909e-1_f64 * t930 * t5040 - 0.57954409931925052364e-1_f64 * t930 * t5043 + 0.779739765264702906e1_f64 * t2812 * t5046;
    t5049
}
