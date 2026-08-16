//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 520/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk520(t5399: f64, t65: f64, t1410: f64, t1426: f64, t2267: f64, t5392: f64, t43: f64, t5398: f64, t48: f64, t480: f64, t2274: f64, t55: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5400 = t5399 * t65;
    let t5403 = t1410 * t1426;
    let t5408 = t2267 * t5392;
    let t5411 = t43 * t5398;
    let t5415 = 1.0_f64 / t48 / t480;
    let t5416 = sigma2 * t5415;
    let t5421 = t2274 * t5392;
    let t5424 = t55 * t5398;
    (t5400, t5403, t5408, t5411, t5415, t5416, t5421, t5424)
}
