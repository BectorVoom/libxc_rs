//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2292/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2292(t16558: f64, t3: f64, t17677: f64, t17705: f64, t1933: f64, t1937: f64, t23419: f64, t88575: f64, t88577: f64, t88582: f64, t88604: f64, t88622: f64, t88625: f64, t88636: f64, t88645: f64) -> f64 {
    let t99767 = t3 * t16558;
    let t99772 = t23419 * t17705 / 1152.0_f64 + t88575 - t88577 + t88582 + t88604 + t23419 * t17677 / 1152.0_f64 + 0.10093189023535097714e-3_f64 * t1933 * t99767 * t1937 - t88622 + t88625 + t88636 - t88645 / 3456.0_f64;
    t99772
}
