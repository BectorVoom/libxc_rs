//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2344/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2344<F: Float>(t3795: F, t91388: F, t26233: F, t3853: F, t80886: F, t80889: F, t80900: F, t91354: F, t91357: F, t91359: F, t91362: F, t91365: F, t91366: F, t91370: F, t91374: F, t91378: F, t91381: F, t91383: F, t91384: F, t91387: F) -> F {
    let t91389 = t91388 * t3795;
    let t91391 = t26233 * t3853;
    let t91393 = -t80886 - F::cast_from(0.59347951458386374556e-1_f64) * t80889 - F::cast_from(0.48447307312968469024e-2_f64) * t91354 - t91357 + t91359 - t91362 / F::cast_from(256.0_f64) - t91365 - t80900 - t91366 / F::cast_from(48.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t91370 - F::cast_from(0.20186378047070195427e-3_f64) * t91374 + F::cast_from(0.40372756094140390854e-3_f64) * t91378 + F::cast_from(0.80745512188280781708e-3_f64) * t91381 - t91383 - t91384 / F::cast_from(1536.0_f64) - t91387 + t91389 / F::cast_from(768.0_f64) - t91391 / F::cast_from(1536.0_f64);
    t91393
}
