//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1206/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1206(t7620: f64, t828: f64, t2427: f64, t2529: f64, t7341: f64, t824: f64, t7710: f64, t809: f64, t2485: f64, t2517: f64, t23789: f64, t23942: f64, t24335: f64, t24704: f64, t24708: f64, t24712: f64, t24715: f64, t24718: f64, t24721: f64, t2476: f64, t2513: f64, t2521: f64, t2525: f64, t2531: f64, t7734: f64, t7744: f64, t7753: f64, t7814: f64, t7817: f64, t7820: f64, t819: f64, t829: f64, t837: f64, t838: f64) -> f64 {
    let t24889 = t7620 * t828;
    let t24892 = t2427 * t2529;
    let t24895 = t824 * t7341;
    let t24906 = t7710 * t809;
    let t24911 = t2485 * t2517;
    let t24917 = 0.23392893589820816284e1_f64 * t24889 * t838 - t24704 - t24708 - t24712 + t24715 + t24718 - 0.70178680769462448852e1_f64 * t24892 * t2531 - 0.4155781415850207192e3_f64 * t24895 * t7814 + 0.6233672123775310788e3_f64 * t7753 * t23789 * t2476 + 0.23392893589820816284e1_f64 * t2525 * t7817 + 0.58482233974552040708e0_f64 * t829 * t23942 * t837 + 4.0_f64 * t24906 * t819 + 6.0_f64 * t7820 * t2513 + 0.19298809906722418784e3_f64 * t24911 * t2521 + t24721 - 0.19751789702565206229e-1_f64 * t24335 - 24.0_f64 * t7744 * t7734;
    t24917
}
