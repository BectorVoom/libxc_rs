//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3365/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3365<F: Float>(t41329: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> F {
    let t63412 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63274 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t63276 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t63278 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t63281 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t63285 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t63290 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63293 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t63299 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t63304 - F::cast_from(8.0_f64) * t63308 + t41329 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t51967;
    t63412
}
