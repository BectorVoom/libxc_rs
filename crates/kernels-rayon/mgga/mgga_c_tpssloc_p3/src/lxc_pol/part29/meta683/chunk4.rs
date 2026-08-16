//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2318/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2318(t24683: f64, t24746: f64, t8027: f64, t4928: f64, t52: f64, t2132: f64, t8040: f64, t86292: f64, t15564: f64, t2136: f64, t23413: f64, t86262: f64, t86266: f64, t86269: f64, t86273: f64, t86275: f64, t86278: f64, t86327: f64) -> (f64, f64) {
    let t95480 = 0.16149102437656156342e-2_f64 * t8027 * t24683 * t24746;
    let t95484 = t52 * t4928;
    let t95487 = 0.20186378047070195428e-3_f64 * t2132 * t95484 * t24746;
    let t95491 = 0.20186378047070195428e-3_f64 * t86292 * t8040;
    let t95492 = t86327 * t15564 / 2304.0_f64 + 0.80745512188280781712e-3_f64 * t8027 * t23413 * t2136 + t95480 - 0.20186378047070195428e-3_f64 * t86262 + 0.20186378047070195428e-3_f64 * t86266 - 0.10093189023535097714e-3_f64 * t86269 - t95487 + t86273 / 2304.0_f64 - t86275 / 3456.0_f64 + t86278 - t95491;
    (t95484, t95492)
}
