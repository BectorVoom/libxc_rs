//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 928/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk928(t1429: f64, t2365: f64, t2366: f64, t31747: f64, t34777: f64, t901: f64, t35106: f64, t41809: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t41958 = t1429 * t2365 * t2366 * t31747;
    let t41960 = t34777 * t901;
    let t41962 = t35106 * t901;
    let t41965 = t41809 * t475;
    (t41958, t41960, t41962, t41965)
}
