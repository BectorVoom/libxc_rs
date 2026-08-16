//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1071/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1071<F: Float>(t2: F, t33300: F, t458: F, t9965: F, t13682: F, t13683: F, t192: F, t2506: F, t3917: F, t41482: F, t41794: F, t41837: F, t41841: F, t41849: F, t42059: F, t42192: F, t42194: F, t42206: F, t42207: F, t42212: F, t42214: F, t42216: F, t462: F, t743: F, t92: F, t9896: F) -> F {
    let t42218 = t33300 * t2;
    let t42227 = t458 * t9965;
    let t42229 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42192 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t42194 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t13682 * t13683 * t42059 - F::cast_from(12.0_f64) * t462 * t3917 * t41482 - F::cast_from(4.0_f64) * t462 * t9896 * t41837 + t42206 + F::cast_from(112.0_f64) / F::cast_from(27.0_f64) * t42207 - t92 * t192 * t743 * t41794 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t42212 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42214 + F::cast_from(8.0_f64) * t42216 + F::cast_from(24.0_f64) * t92 * t192 * t42218 * t41849 + F::cast_from(6.0_f64) * t92 * t192 * t2506 * t41841 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42227;
    t42229
}
