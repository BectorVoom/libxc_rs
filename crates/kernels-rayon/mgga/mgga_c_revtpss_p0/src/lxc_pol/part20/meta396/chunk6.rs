//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1463/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1463(t2944: f64, t41245: f64, t41250: f64, t41255: f64, t41260: f64, t41265: f64, t41267: f64, t41273: f64, t41275: f64, t41279: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64) -> (f64, f64) {
    let t41668 = t2944 * t2944;
    let t41672 = 0.16979925925925925926e1_f64 * t41245;
    let t41686 = t41672 - 0.27785333333333333334e0_f64 * t41250 + 0.83356e0_f64 * t41255 - 0.13892666666666666667e0_f64 * t41260 + 0.125034e1_f64 * t41265 - 0.166712e1_f64 * t41267 + 0.55570666666666666666e0_f64 * t41273 + 0.166712e1_f64 * t41275 - 0.125034e1_f64 * t41279 + 0.13892666666666666667e1_f64 * t41281 - 0.55570666666666666668e0_f64 * t41283 - 0.69463333333333333334e0_f64 * t41285 - 0.23154444444444444445e0_f64 * t41287 + 0.27785333333333333333e0_f64 * t41289;
    (t41668, t41686)
}
