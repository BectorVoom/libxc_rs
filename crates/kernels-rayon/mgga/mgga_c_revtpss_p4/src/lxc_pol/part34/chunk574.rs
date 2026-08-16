//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 574/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk574(t45: f64, t57: f64, t4399: f64, t5819: f64, t5825: f64, t766: f64, t80: f64, t770: f64, t83: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t5948 = 0.11696447245269292414e1_f64 * t4399;
    let t5954 = piecewise3(t151, 0.0_f64, -2.0_f64 / 9.0_f64 * t80 * t5819 + 2.0_f64 / 3.0_f64 * t766 * t5825);
    let t5960 = piecewise3(t155, 0.0_f64, -2.0_f64 / 9.0_f64 * t83 * t5819 - 2.0_f64 / 3.0_f64 * t770 * t5825);
    let t5962 = t5954 / 2.0_f64 + t5960 / 2.0_f64;
    (t5948, t5962)
}
