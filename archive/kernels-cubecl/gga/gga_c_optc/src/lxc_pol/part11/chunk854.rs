//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 854/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk854<F: Float>(t16372: F, t696: F, t127: F, t16370: F, t5: F, t675: F, t116: F, t16287: F, t627: F, t13330: F, t13364: F, t13366: F, t13368: F, t13373: F, t13376: F, t13378: F, t13380: F, t13390: F, t673: F, t686: F, t695: F) -> (F, F, F, F) {
    let t16500 = t696 * t16372;
    let t16504 = t5 * t16370 * t127;
    let t16505 = t675 * t16504;
    let t16513 = t627 * t116 * t16287;
    let t16521 = -F::cast_from(0.15114211337509259186e-1_f64) * t695 * t16500 - F::cast_from(0.86931614897887578546e-1_f64) * t673 * t16505 - F::cast_from(0.36511278257112782988e1_f64) * t13330 + F::cast_from(0.4231979174502592572e0_f64) * t13364 - F::cast_from(0.2115989587251296286e1_f64) * t13366 + F::cast_from(0.12170426085704260996e1_f64) * t13368 - F::cast_from(0.17386322979577515709e0_f64) * t686 * t16513 - F::cast_from(0.6347968761753888858e0_f64) * t13373 + F::cast_from(0.60852130428521304981e0_f64) * t13376 + F::cast_from(0.60852130428521304981e0_f64) * t13378 - F::cast_from(0.12170426085704260996e1_f64) * t13380 + F::cast_from(0.2115989587251296286e0_f64) * t13390;
    (t16500, t16505, t16513, t16521)
}
