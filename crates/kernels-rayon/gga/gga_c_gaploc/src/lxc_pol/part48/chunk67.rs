//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 67/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk67(t141: f64, t165: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t225 = 2.0_f64 <= zeta_threshold;
    let t228 = 0.0_f64 <= zeta_threshold;
    let t280 = piecewise3(t225, t141, t165);
    let t281 = piecewise3(t228, t141, 0.0_f64);
    let t283 = t280 / 2.0_f64 + t281 / 2.0_f64;
    let t284 = t283 * t283;
    let t286 = 1.0_f64 / t284 / t283;
    (t283, t284, t286)
}
