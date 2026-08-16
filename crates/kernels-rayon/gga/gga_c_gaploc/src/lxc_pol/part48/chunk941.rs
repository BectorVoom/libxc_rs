//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 941/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk941(t10283: f64, t2902: f64, t13578: f64, t16710: f64, t841: f64, t13483: f64, t1382: f64, t605: f64, t23575: f64, t3638: f64, t13585: f64, t5552: f64) -> (f64, f64, f64, f64, f64) {
    let t45978 = 2.0_f64 * t10283 * t2902;
    let t45983 = 24.0_f64 * t16710 * t13578 * t841;
    let t45986 = 2.0_f64 * t1382 * t13483 * t605;
    let t45988 = 2.0_f64 * t23575 * t3638;
    let t45990 = 2.0_f64 * t5552 * t13585;
    (t45978, t45983, t45986, t45988, t45990)
}
