//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2310/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2310<F: Float>(t24661: F, t27491: F, t15617: F, t24655: F, t24664: F, t24670: F, t27711: F, t7331: F, t7345: F, t86174: F, t86176: F, t86184: F, t86234: F, t95320: F, t95323: F, t95327: F, t95334: F, t95335: F) -> F {
    let t95340 = t24661 * t27491;
    let t95343 = -t7345 * t15617 / F::cast_from(384.0_f64) + t95320 - F::cast_from(0.80745512188280781712e-3_f64) * t27711 * t24655 + F::cast_from(0.16149102437656156342e-2_f64) * t95323 * t7331 - F::cast_from(0.16149102437656156342e-2_f64) * t95327 * t24664 + F::cast_from(0.80745512188280781712e-3_f64) * t95327 * t24670 - t95334 - t95335 / F::cast_from(6912.0_f64) - t86174 / F::cast_from(2304.0_f64) - t86176 / F::cast_from(3456.0_f64) + t86184 / F::cast_from(648.0_f64) - F::cast_from(0.40372756094140390856e-3_f64) * t86234 * t95340;
    t95343
}
