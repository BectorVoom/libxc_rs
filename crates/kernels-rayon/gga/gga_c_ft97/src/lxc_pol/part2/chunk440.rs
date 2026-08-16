//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 440/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk440(t2459: f64, t676: f64, t27: f64, t89: f64, t2335: f64, t2339: f64, t2342: f64, t2352: f64, t2357: f64, t2364: f64, t2368: f64, t2376: f64) -> (f64, f64, f64) {
    let t2460 = t676 * t2459;
    let t2462 = t89 * t27 * t2460;
    let t2464 = t2335 + t2339 + t2342 - t2352 / 27.0_f64 + t2357 / 9.0_f64 + t2364 / 9.0_f64 - t2368 / 18.0_f64 + t2376 / 3.0_f64 - t2462 / 6.0_f64;
    (t2460, t2462, t2464)
}
