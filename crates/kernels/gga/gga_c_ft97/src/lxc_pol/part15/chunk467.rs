//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 467/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk467<F: Float>(t2092: F, t3497: F, t3513: F, t462: F, t4759: F, t4762: F, t4765: F, t4768: F, t4772: F, t4776: F, t92: F, t579: F, t91: F) -> (F, F) {
    let t4778 = t2092 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3497 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3513 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t4759 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t4762 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t4765 - t462 * t4768 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t92 * t4772 - t92 * t4776;
    let t4780 = t91 * t579 * t4778;
    (t4778, t4780)
}
