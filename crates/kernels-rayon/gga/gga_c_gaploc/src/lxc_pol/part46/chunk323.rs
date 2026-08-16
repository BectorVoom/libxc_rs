//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 323/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk323(t747: f64, t952: f64, t841: f64, t977: f64, t1628: f64, t973: f64, t2027: f64, t959: f64, t701: f64, t733: f64, t2365: f64, t2022: f64) -> (f64, f64, f64, f64, f64) {
    let t2592 = t952 * t747;
    let t2595 = t977 * t841;
    let t2598 = t1628 * t973;
    let t2601 = t2027 * t959;
    let t2603 = t733 * t701;
    let t2604 = t2365 * t2603;
    let t2605 = t2022 * t2604;
    (t2592, t2595, t2598, t2601, t2605)
}
