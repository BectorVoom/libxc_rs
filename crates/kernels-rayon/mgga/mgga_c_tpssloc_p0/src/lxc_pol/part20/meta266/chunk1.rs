//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1422/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1422(t10321: f64, t908: f64, t136: f64, t10295: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10311: f64, t10314: f64, t10318: f64, t10320: f64) -> (f64, f64, f64) {
    let t10322 = t908 * t10321;
    let t10323 = t136 * t10322;
    let t10325 = t10295 + 5.0_f64 / 9.0_f64 * t10296 - t10298 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t10300 - t10302 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t10307 - t10311 / 3.0_f64 + t10314 / 6.0_f64 + t10318 - t10320 + t10323 / 6.0_f64;
    (t10322, t10323, t10325)
}
