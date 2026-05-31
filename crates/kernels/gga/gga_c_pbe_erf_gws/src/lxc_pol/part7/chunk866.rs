//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 866/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk866<F: Float>(t16641: F, t1820: F, t5539: F, t7669: F, t4897: F, t5137: F, t639: F, t5342: F, t586: F, t1812: F, t4913: F, t5142: F) -> (F, F, F, F, F, F) {
    let t16642 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t16641;
    let t16644 = t1820 * t7669 * t5539;
    let t16645 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t16644;
    let t16647 = t639 * t5137 * t4897;
    let t16648 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16647;
    let t16649 = t5342 * t586;
    let t16651 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t16649 * t1812;
    let t16653 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t4913 * t5142;
    (t16642, t16645, t16648, t16649, t16651, t16653)
}
