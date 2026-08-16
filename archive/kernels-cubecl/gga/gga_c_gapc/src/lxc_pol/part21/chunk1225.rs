//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1225/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1225<F: Float>(t11388: F, t3065: F, t11479: F, t1912: F, t5285: F, t11326: F, t8885: F, t35135: F, t35137: F, t35141: F, t35143: F, t35146: F, t35149: F, t35152: F, t35155: F) -> F {
    let t35157 = t11388 * t3065;
    let t35160 = t5285 * t11479 * t1912;
    let t35162 = t11326 * t8885;
    let t35164 = -F::cast_from(0.21642471925239962898e-3_f64) * t35135 - F::cast_from(0.16217772716043213195e-2_f64) * t35137 - F::cast_from(0.30775559784820528656e-8_f64) * t35141 - F::cast_from(0.13506074236995523433e-5_f64) * t35143 + F::cast_from(0.5686343261418565457e-6_f64) * t35146 - F::cast_from(0.32228090843368550272e-8_f64) * t35149 + F::cast_from(0.168651611569216142e-8_f64) * t35152 + F::cast_from(0.27665946779727057415e-8_f64) * t35155 + F::cast_from(0.49522272202316919254e-5_f64) * t35157 + F::cast_from(0.16908181191593721013e-5_f64) * t35160 - F::cast_from(0.40096157891080460192e-6_f64) * t35162;
    t35164
}
