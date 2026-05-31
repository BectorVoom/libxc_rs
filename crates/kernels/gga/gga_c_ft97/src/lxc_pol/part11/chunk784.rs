//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 784/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk784<F: Float>(t1775: F, t2775: F, t10589: F, t10591: F, t10594: F, t10595: F, t10597: F, t10600: F, t10604: F, t10607: F, t10611: F, t10614: F, t10617: F, t462: F, t92: F) -> F {
    let t10619 = t1775 * t2775;
    let t10621 = t10589 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10591 - t10594 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10595 - F::cast_from(2.0_f64) * t462 * t10597 + F::cast_from(2.0_f64) * t462 * t10600 - F::cast_from(2.0_f64) * t462 * t10604 - F::cast_from(2.0_f64) * t462 * t10607 - t92 * t10611 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t10614 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10617 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10619;
    t10621
}
