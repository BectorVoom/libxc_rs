//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1487/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1487(t57: f64, t13312: f64, t14413: f64, t14416: f64, t2251: f64, t2258: f64, t4384: f64, t606: f64, t81: f64, t14412: f64, t162: f64, t187: f64, t2615: f64, t4311: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t14424 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t14413 * t2251 + 8.0_f64 / 9.0_f64 * t14416 * t606 + 4.0_f64 / 9.0_f64 * t4384 * t2258 - 4.0_f64 / 3.0_f64 * t81 * t13312);
    let t14425 = t14412 + t14424;
    let t14426 = t14425 * t162;
    let t14428 = 0.19751673498613801407e-1_f64 * t14426 * t187;
    let t14433 = 8.0_f64 * t4311 * t2615;
    (t14425, t14428, t14433)
}
