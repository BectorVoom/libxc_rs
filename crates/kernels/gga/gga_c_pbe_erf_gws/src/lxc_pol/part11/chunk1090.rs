//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1090/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1090<F: Float>(t185: F, t186: F, t47428: F, t47458: F, t47487: F, t47500: F, t598: F, t30407: F, t3465: F, t3553: F, t5522: F, t639: F) -> (F, F, F) {
    let t47506 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t186 * t598 * (t47428 + t47458 + t47487 + t47500);
    let t47507 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t30407;
    let t47511 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t639 * t5522 * t3465 * t3553;
    (t47506, t47507, t47511)
}
