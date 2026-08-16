//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 846/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk846<F: Float>(t34511: F, t488: F, t10969: F, t7274: F, t32066: F, t32093: F, t34373: F, t34377: F, t34382: F, t34387: F, t34391: F, t34395: F, t34399: F, t34403: F, t34408: F) -> (F, F, F) {
    let t34512 = t488 * t34511;
    let t34514 = t10969 * t7274;
    let t34524 = t34373 / F::cast_from(2.0_f64) + t32066 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t34377 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t34382 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t34387 - t34391 / F::cast_from(6.0_f64) - t32093 - t34395 / F::cast_from(9.0_f64) - t34399 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t34403 + t34408 / F::cast_from(12.0_f64);
    (t34512, t34514, t34524)
}
