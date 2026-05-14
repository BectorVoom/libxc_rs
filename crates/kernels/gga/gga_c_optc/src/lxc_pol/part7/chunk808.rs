//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 808/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk808<F: Float>(t2724: F, t8152: F, t2721: F, t2633: F, t7178: F, t914: F, t2596: F, t894: F, t2264: F, t2723: F, t3836: F, t2722: F, t8044: F, t3608: F, t2725: F, t2812: F, t2814: F, t8134: F, t8135: F, t8138: F, t8140: F, t8145: F, t8149: F, t930: F, t953: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8153 = t8152 * t2724;
    let t8154 = t2721 * t8153;
    let t8156 = t2633 * t7178;
    let t8157 = t914 * t8156;
    let t8160 = t2596 * t7178;
    let t8161 = t894 * t8160;
    let t8164 = t2723 * t2264;
    let t8165 = t3836 * t8164;
    let t8168 = t2722 * t8044;
    let t8171 = t3608 * t8164;
    let t8174 = -0.8987985586528718635e4 * t8134 * t8135 - 0.33587136305576131526e-2 * t8138 - 0.12475836244235246496e3 * t8140 * t2814 + 0.1559479530529405812e2 * t8145 - 0.60587206808032502059e1 * t8149 * t2725 + 0.75734008510040627575e0 * t8154 + 0.11590881986385010473e0 * t930 * t8157 + 0.25190352229182098644e-1 * t953 * t8161 + 0.1949349413161757265e2 * t2812 * t8165 + 0.11360101276506094136e1 * t2721 * t8168 + 0.15146801702008125515e1 * t2721 * t8171;
    (t8153, t8156, t8157, t8160, t8161, t8164, t8165, t8168, t8171, t8174)
}
