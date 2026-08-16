//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2310/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2310(t24661: f64, t27491: f64, t15617: f64, t24655: f64, t24664: f64, t24670: f64, t27711: f64, t7331: f64, t7345: f64, t86174: f64, t86176: f64, t86184: f64, t86234: f64, t95320: f64, t95323: f64, t95327: f64, t95334: f64, t95335: f64) -> f64 {
    let t95340 = t24661 * t27491;
    let t95343 = -t7345 * t15617 / 384.0_f64 + t95320 - 0.80745512188280781712e-3_f64 * t27711 * t24655 + 0.16149102437656156342e-2_f64 * t95323 * t7331 - 0.16149102437656156342e-2_f64 * t95327 * t24664 + 0.80745512188280781712e-3_f64 * t95327 * t24670 - t95334 - t95335 / 6912.0_f64 - t86174 / 2304.0_f64 - t86176 / 3456.0_f64 + t86184 / 648.0_f64 - 0.40372756094140390856e-3_f64 * t86234 * t95340;
    t95343
}
