//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3456/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3456<F: Float>(t42013: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> F {
    let t65012 = F::cast_from(0.33333333333333333334e-1_f64) * t63274 - F::cast_from(0.11111111111111111111e-1_f64) * t63276 + F::cast_from(0.37037037037037037037e-2_f64) * t63278 - F::cast_from(0.11111111111111111111e-1_f64) * t63281 - F::cast_from(0.55555555555555555555e-2_f64) * t63285 - F::cast_from(0.92592592592592592592e-2_f64) * t63290 + F::cast_from(0.33333333333333333334e-1_f64) * t63293 + F::cast_from(0.16666666666666666667e-1_f64) * t63299 + F::cast_from(0.11111111111111111111e0_f64) * t63304 - F::cast_from(0.19999999999999999999e0_f64) * t63308 + t42013 + F::cast_from(0.55555555555555555556e-2_f64) * t51967;
    t65012
}
