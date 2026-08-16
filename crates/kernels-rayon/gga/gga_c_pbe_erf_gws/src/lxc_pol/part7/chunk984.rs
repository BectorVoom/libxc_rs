//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 984/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk984(t108: f64, t1403: f64, t1407: f64, t1413: f64, t1416: f64, t1523: f64, t1528: f64, t16648: f64, t16651: f64, t16653: f64, t16655: f64, t16657: f64, t16662: f64, t16665: f64, t16668: f64, t16669: f64, t16679: f64, t16746: f64, t16756: f64, t16758: f64, t16973: f64, t16978: f64, t16986: f64, t18160: f64, t267: f64, t4360: f64, t4373: f64, t476: f64, t478: f64, t5189: f64, t5196: f64, t726: f64, t728: f64, t92: f64, t93: f64) -> f64 {
    let t18188 = -t16648 + t16651 + t16653 + t16655 + t16657 + t16662 + t16665 - t16668 + t16756 - 8.0_f64 / 45.0_f64 * t18160 - (-40.0_f64 / 81.0_f64 * t1523 * t16669 + 80.0_f64 / 9.0_f64 * t476 * t1403 * t1407 + 20.0_f64 / 3.0_f64 * t92 * t16679 + 80.0_f64 / 9.0_f64 * t5189 * t4360 + 4.0_f64 / 3.0_f64 * t726 * t16746 - 40.0_f64 / 81.0_f64 * t1528 * t16973 + 80.0_f64 / 9.0_f64 * t478 * t1413 * t1416 + 20.0_f64 / 3.0_f64 * t93 * t16986 + 80.0_f64 / 9.0_f64 * t5196 * t4373 + 4.0_f64 / 3.0_f64 * t728 * t16978) * t108 * t267 / 15.0_f64 - t16758;
    t18188
}
