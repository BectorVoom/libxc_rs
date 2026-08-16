//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 883/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk883<F: Float>(t255: F, t42123: F, t41950: F, t761: F, t9577: F, t259: F, t41743: F, t89: F, t327: F, t41446: F, t170: F, t328: F, t39600: F) -> (F, F, F, F, F, F) {
    let t42517 = t42123 * t255;
    let t42759 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t41950;
    let t42859 = t761 * t9577;
    let t42928 = F::cast_from(280.0_f64) / F::cast_from(243.0_f64) * t89 * t41743 * t259;
    let t43050 = t327 * t41446;
    let t43084 = F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t170 * t39600 * t328;
    (t42517, t42759, t42859, t42928, t43050, t43084)
}
