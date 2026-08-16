//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2344/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2344(t3795: f64, t91388: f64, t26233: f64, t3853: f64, t80886: f64, t80889: f64, t80900: f64, t91354: f64, t91357: f64, t91359: f64, t91362: f64, t91365: f64, t91366: f64, t91370: f64, t91374: f64, t91378: f64, t91381: f64, t91383: f64, t91384: f64, t91387: f64) -> f64 {
    let t91389 = t91388 * t3795;
    let t91391 = t26233 * t3853;
    let t91393 = -t80886 - 0.59347951458386374556e-1_f64 * t80889 - 0.48447307312968469024e-2_f64 * t91354 - t91357 + t91359 - t91362 / 256.0_f64 - t91365 - t80900 - t91366 / 48.0_f64 - 0.12111826828242117256e-2_f64 * t91370 - 0.20186378047070195427e-3_f64 * t91374 + 0.40372756094140390854e-3_f64 * t91378 + 0.80745512188280781708e-3_f64 * t91381 - t91383 - t91384 / 1536.0_f64 - t91387 + t91389 / 768.0_f64 - t91391 / 1536.0_f64;
    t91393
}
