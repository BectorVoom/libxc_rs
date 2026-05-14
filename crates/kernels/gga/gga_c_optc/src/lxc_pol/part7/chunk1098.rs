//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1098/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1098<F: Float>(t7620: F, t828: F, t2427: F, t2529: F, t7341: F, t824: F, t7710: F, t809: F, t2485: F, t2517: F, t23789: F, t23942: F, t24335: F, t24704: F, t24708: F, t24712: F, t24715: F, t24718: F, t24721: F, t2476: F, t2513: F, t2521: F, t2525: F, t2531: F, t7734: F, t7744: F, t7753: F, t7814: F, t7817: F, t7820: F, t819: F, t829: F, t837: F, t838: F) -> (F,) {
    let t24889 = t7620 * t828;
    let t24892 = t2427 * t2529;
    let t24895 = t824 * t7341;
    let t24906 = t7710 * t809;
    let t24911 = t2485 * t2517;
    let t24917 = 0.23392893589820816284e1 * t24889 * t838 - t24704 - t24708 - t24712 + t24715 + t24718 - 0.70178680769462448852e1 * t24892 * t2531 - 0.4155781415850207192e3 * t24895 * t7814 + 0.6233672123775310788e3 * t7753 * t23789 * t2476 + 0.23392893589820816284e1 * t2525 * t7817 + 0.58482233974552040708e0 * t829 * t23942 * t837 + 4.0 * t24906 * t819 + 6.0 * t7820 * t2513 + 0.19298809906722418784e3 * t24911 * t2521 + t24721 - 0.19751789702565206229e-1 * t24335 - 24.0 * t7744 * t7734;
    (t24917,)
}
