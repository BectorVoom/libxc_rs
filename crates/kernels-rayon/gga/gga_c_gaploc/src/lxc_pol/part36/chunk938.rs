//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 938/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk938(t43069: f64, t43071: f64, t43072: f64, t43073: f64, t43075: f64, t43076: f64, t43077: f64, t43078: f64, t43079: f64, t43080: f64, t739: f64, t2508: f64, t2717: f64, t3433: f64) -> (f64, f64, f64) {
    let t43081 = t43069 - t43071 + t43072 - t43073 / 2.0_f64 + t43075 + t43076 - t43077 + t43078 - t43079 - t43080;
    let t43082 = t739 * t43081;
    let t43087 = t2508 * t2717 * t3433;
    (t43081, t43082, t43087)
}
