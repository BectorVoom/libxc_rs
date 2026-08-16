//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1930/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1930(t23270: f64, t28267: f64, t22986: f64, t225: f64, t258: f64, t5631: f64, t214: f64, t1880: f64, t5544: f64, t6554: f64, t6553: f64, t6552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28268 = t23270 * t28267;
    let t28269 = t22986 * t28268;
    let t28272 = t5631 * t225 * t258;
    let t28273 = t214 * t28272;
    let t28274 = t1880 * t28273;
    let t28276 = t6554 * t5544;
    let t28277 = t6553 * t28276;
    let t28278 = t6552 * t28277;
    (t28268, t28269, t28272, t28273, t28274, t28276, t28277, t28278)
}
