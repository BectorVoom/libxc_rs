//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 294/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk294(t501: f64, t892: f64, t605: f64, t921: f64, t589: f64, t913: f64, t587: f64, t123: f64, t160: f64, t90: f64) -> (f64, f64, f64, f64) {
    let t2355 = t892 * t501;
    let t2358 = t921 * t605;
    let t2361 = t589 * t913;
    let t2362 = t587 * t2361;
    let t2365 = t90 * t123 * t160;
    (t2355, t2358, t2362, t2365)
}
