//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2319/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2319(t18324: f64, t7310: f64, t18371: f64, t24741: f64, t29569: f64, t29651: f64, t4954: f64, t7321: f64, t86184: f64, t95320: f64, t95334: f64, t95335: f64, t95352: f64, t95362: f64, t95364: f64, t95365: f64, t95687: f64) -> f64 {
    let t104088 = t7310 * t18324;
    let t104094 = t24741 * t18371;
    let t104101 = -t104088 / 432.0_f64 - 0.16149102437656156342e-2_f64 * t29569 * t7321 + t95320 - t95687 * t4954 / 1152.0_f64 - t104094 / 1728.0_f64 - t95334 - t95335 / 3456.0_f64 + 0.10093189023535097714e-3_f64 * t29651 * t7321 + t86184 / 1296.0_f64 - t95352 + t95362 - t95364 - t95365 / 3456.0_f64;
    t104101
}
