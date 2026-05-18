//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 853/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk853<F: Float>(t2724: F, t8152: F, t2721: F, t2633: F, t7178: F, t914: F, t2596: F, t894: F, t2264: F, t2723: F, t3836: F, t2722: F, t8044: F) -> (F, F, F, F, F, F, F, F, F) {
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
