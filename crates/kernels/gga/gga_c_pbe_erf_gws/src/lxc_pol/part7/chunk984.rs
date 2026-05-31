//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 984/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk984<F: Float>(t108: F, t1403: F, t1407: F, t1413: F, t1416: F, t1523: F, t1528: F, t16648: F, t16651: F, t16653: F, t16655: F, t16657: F, t16662: F, t16665: F, t16668: F, t16669: F, t16679: F, t16746: F, t16756: F, t16758: F, t16973: F, t16978: F, t16986: F, t18160: F, t267: F, t4360: F, t4373: F, t476: F, t478: F, t5189: F, t5196: F, t726: F, t728: F, t92: F, t93: F) -> F {
    let t18188 = -t16648 + t16651 + t16653 + t16655 + t16657 + t16662 + t16665 - t16668 + t16756 - F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t18160 - (-F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1523 * t16669 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t476 * t1403 * t1407 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t92 * t16679 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t5189 * t4360 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t16746 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1528 * t16973 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t478 * t1413 * t1416 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t93 * t16986 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t5196 * t4373 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t16978) * t108 * t267 / F::cast_from(15.0_f64) - t16758;
    t18188
}
