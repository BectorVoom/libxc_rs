//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1448/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1448(t20245: f64, t337: f64, t104126: f64, t104128: f64, t104139: f64, t104142: f64, t104150: f64, t104153: f64, t104181: f64, t104184: f64, t104187: f64, t131: f64, t22280: f64, t22284: f64, t22288: f64, t24741: f64, t27704: f64, t29569: f64, t29625: f64, t467: f64, t8035: f64, t86324: f64, t86327: f64, t95450: f64) -> (f64, f64) {
    let t109535 = t20245 * t337;
    let t109553 = -t104126 / 72.0_f64 + 19.0_f64 / 432.0_f64 * t104128 - 0.30279567070605293142e-3_f64 * t27704 * t29625 - 0.48447307312968469026e-2_f64 * t29569 * t8035 - 77.0_f64 / 162.0_f64 * t109535 * t131 * t467 + 11.0_f64 / 108.0_f64 * t104139 + 0.21801288290835811062e-1_f64 * t104142 + t104150 / 54.0_f64 + 0.48447307312968469026e-2_f64 * t104153 + 0.60559134141210586284e-3_f64 * t104181 - 0.30279567070605293142e-3_f64 * t104184 + t95450 / 54.0_f64 - 0.60559134141210586284e-3_f64 * t104187 - t86324 * t22280 / 384.0_f64 + t86327 * t22284 / 768.0_f64 - t24741 * t22288 / 384.0_f64;
    (t109535, t109553)
}
