//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1325/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1325(t57: f64, t202: f64, t635: f64, t10326: f64, t10457: f64, t10460: f64, t2251: f64, t2258: f64, t2382: f64, t39443: f64, t39449: f64, t39457: f64, t81: f64, zeta_threshold: f64) -> f64 {
    let t155 = t57 <= zeta_threshold;
    let t39840 = 1.0_f64 / t202 / t635;
    let t39853 = piecewise3(t155, 0.0_f64, 40.0_f64 / 81.0_f64 * t39840 * t39443 + 16.0_f64 / 9.0_f64 * t10457 * t2251 * t2258 + 4.0_f64 / 3.0_f64 * t2382 * t39449 + 16.0_f64 / 9.0_f64 * t10460 * t10326 - 4.0_f64 / 3.0_f64 * t81 * t39457);
    t39853
}
