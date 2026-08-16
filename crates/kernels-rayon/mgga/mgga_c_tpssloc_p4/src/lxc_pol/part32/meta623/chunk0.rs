//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2030/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2030(t11820: f64, t7339: f64, t2122: f64, t7319: f64, t1235: f64, t225: f64, t461: f64, t11553: f64, t2121: f64, t2123: f64, t7288: f64, t85660: f64) -> (f64, f64, f64, f64, f64) {
    let t86350 = t7339 * t11820;
    let t86403 = t7319 * t2122;
    let t86415 = t461 * t1235 * t225;
    let t86451 = 0.30461741978670859935e-2_f64 * t2121 * t11553 * t2123;
    let t86473 = t85660 * t7288;
    (t86350, t86403, t86415, t86451, t86473)
}
