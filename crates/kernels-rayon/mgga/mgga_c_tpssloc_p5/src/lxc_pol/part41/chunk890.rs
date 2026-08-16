//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 890/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk890(t1401: f64, t1458: f64, t2199: f64, t3941: f64, t5371: f64, t577: f64, t8207: f64, t8273: f64, t8283: f64, t8294: f64, t590: f64, t60: f64) -> (f64, f64) {
    let t8299 = 0.45e1_f64 * t8283 * t577 + 0.135e2_f64 * t8207 * t1458 + 0.135e2_f64 * t5371 * t2199 + 27.0_f64 * t3941 * t8294 + 0.135e2_f64 * t1401 * t8273;
    let t8705 = 1.0_f64 / t60 / t590;
    (t8299, t8705)
}
