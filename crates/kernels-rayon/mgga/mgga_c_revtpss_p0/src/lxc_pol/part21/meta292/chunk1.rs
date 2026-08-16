//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1536/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1536(t57: f64, t202: f64, t2382: f64, t606: f64, t10326: f64, t10356: f64, t2258: f64, t81: f64, t10455: f64, t150: f64, t190: f64, t80: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t10457 = 1.0_f64 / t202 / t57;
    let t10460 = t2382 * t606;
    let t10466 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t10457 * t10356 + 4.0_f64 / 3.0_f64 * t10460 * t2258 - 4.0_f64 / 3.0_f64 * t81 * t10326);
    let t10467 = t10455 + t10466;
    let t10468 = t150 * t10467;
    let t10469 = t10468 * t190;
    let t10472 = t80 * t606;
    (t10457, t10467, t10468, t10469, t10472)
}
