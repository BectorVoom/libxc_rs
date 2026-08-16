//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 865/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk865(t45: f64, t57: f64, t2375: f64, t606: f64, t10326: f64, t10356: f64, t10446: f64, t2258: f64, t78: f64, t202: f64, t2382: f64, t81: f64, t150: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10449 = t2375 * t606;
    let t10455 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t10446 * t10356 + 4.0_f64 / 3.0_f64 * t10449 * t2258 + 4.0_f64 / 3.0_f64 * t78 * t10326);
    let t10457 = 1.0_f64 / t202 / t57;
    let t10460 = t2382 * t606;
    let t10466 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t10457 * t10356 + 4.0_f64 / 3.0_f64 * t10460 * t2258 - 4.0_f64 / 3.0_f64 * t81 * t10326);
    let t10467 = t10455 + t10466;
    let t10468 = t150 * t10467;
    (t10467, t10468)
}
