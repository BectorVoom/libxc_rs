//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 542/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk542(t655: f64, t130: f64, t146: f64, t2289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2303 = t655 * t655;
    let t2304 = 1.0_f64 / t2303;
    let t2305 = t130 * t2304;
    let t2306 = t146 * t146;
    let t2307 = 1.0_f64 / t2306;
    let t2308 = t2289 * t2307;
    let t2310 = 0.16081979498692535067e2_f64 * t2305 * t2308;
    (t2303, t2304, t2305, t2306, t2307, t2308, t2310)
}
