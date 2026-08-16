//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 854/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk854(t16372: f64, t696: f64, t127: f64, t16370: f64, t5: f64, t675: f64, t116: f64, t16287: f64, t627: f64, t13330: f64, t13364: f64, t13366: f64, t13368: f64, t13373: f64, t13376: f64, t13378: f64, t13380: f64, t13390: f64, t673: f64, t686: f64, t695: f64) -> (f64, f64, f64, f64) {
    let t16500 = t696 * t16372;
    let t16504 = t5 * t16370 * t127;
    let t16505 = t675 * t16504;
    let t16513 = t627 * t116 * t16287;
    let t16521 = -0.15114211337509259186e-1_f64 * t695 * t16500 - 0.86931614897887578546e-1_f64 * t673 * t16505 - 0.36511278257112782988e1_f64 * t13330 + 0.4231979174502592572e0_f64 * t13364 - 0.2115989587251296286e1_f64 * t13366 + 0.12170426085704260996e1_f64 * t13368 - 0.17386322979577515709e0_f64 * t686 * t16513 - 0.6347968761753888858e0_f64 * t13373 + 0.60852130428521304981e0_f64 * t13376 + 0.60852130428521304981e0_f64 * t13378 - 0.12170426085704260996e1_f64 * t13380 + 0.2115989587251296286e0_f64 * t13390;
    (t16500, t16505, t16513, t16521)
}
