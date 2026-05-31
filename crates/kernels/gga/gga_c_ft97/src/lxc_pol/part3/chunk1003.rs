//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1003/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1003<F: Float>(t19526: F, t2882: F, t2881: F, t11593: F, t1901: F, t19479: F, t19482: F, t19484: F, t19487: F, t19491: F, t19494: F, t19497: F, t19501: F, t19504: F, t19508: F, t19511: F, t19514: F, t19519: F, t19523: F) -> F {
    let t19527 = t2882 * t19526;
    let t19528 = t2881 * t19527;
    let t19531 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t19479 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19482 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19484 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t19487 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1901 * t19491 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t19494 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t19497 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t19501 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19504 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t19508 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19511 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11593 * t19514 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t19519 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t19523 + t1901 * t19528 / F::cast_from(9.0_f64);
    t19531
}
