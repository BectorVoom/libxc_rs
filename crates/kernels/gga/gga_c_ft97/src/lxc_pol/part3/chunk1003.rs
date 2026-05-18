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
    let t19531 = F::new(2.0) / F::new(9.0) * t1901 * t19479 - F::new(2.0) / F::new(9.0) * t19482 - F::new(2.0) / F::new(9.0) * t19484 + F::new(4.0) / F::new(9.0) * t1901 * t19487 - F::new(10.0) / F::new(81.0) * t1901 * t19491 + F::new(2.0) / F::new(9.0) * t1901 * t19494 + F::new(4.0) / F::new(9.0) * t1901 * t19497 - F::new(4.0) / F::new(27.0) * t1901 * t19501 - F::new(2.0) / F::new(27.0) * t19504 - F::new(4.0) / F::new(3.0) * t1901 * t19508 - F::new(2.0) / F::new(27.0) * t19511 + F::new(8.0) / F::new(27.0) * t11593 * t19514 - F::new(2.0) / F::new(9.0) * t1901 * t19519 - F::new(4.0) / F::new(3.0) * t1901 * t19523 + t1901 * t19528 / F::new(9.0);
    t19531
}
