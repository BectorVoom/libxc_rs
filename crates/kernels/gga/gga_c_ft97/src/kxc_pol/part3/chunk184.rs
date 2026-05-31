//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 184/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk184<F: Float>(t24: F, t558: F, t586: F, t462: F, t581: F, t583: F, t92: F, t579: F, t91: F, t517: F, t522: F, t561: F) -> (F, F, F, F, F) {
    let t588 = t24 * t586 * t558;
    let t590 = -t581 - t462 * t583 / F::cast_from(3.0_f64) - t92 * t588;
    let t592 = t91 * t579 * t590;
    let t594 = t517 / F::cast_from(9.0_f64);
    let t597 = t592 / F::cast_from(6.0_f64) - t594 - t522 / F::cast_from(9.0_f64) - t561 / F::cast_from(3.0_f64);
    (t588, t590, t592, t594, t597)
}
