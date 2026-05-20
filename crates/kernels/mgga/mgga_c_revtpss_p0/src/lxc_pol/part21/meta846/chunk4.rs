//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3169/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3169<F: Float>(t58225: F, t56248: F, t56252: F, t56256: F, t58202: F, t58207: F, t58209: F, t58211: F, t58214: F, t58217: F, t58220: F, t58223: F) -> F {
    let t58452 = F::new(0.5519e0) * t58225;
    let t58453 = F::new(0.49671e0) * t58202 + F::cast_from(0.10064166666666666667e1_f64) * t56248 + F::new(0.543465e1) * t56252 - F::new(0.36231e1) * t56256 - F::cast_from(0.73586666666666666668e-1_f64) * t58207 - F::new(0.33114e0) * t58209 - F::new(0.99342e0) * t58211 + F::new(0.44152e0) * t58214 + F::new(0.16557e0) * t58217 + F::new(0.149013e1) * t58220 + F::new(0.198684e1) * t58223 + t58452;
    t58453
}
