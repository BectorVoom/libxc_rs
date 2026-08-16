//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 334/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk334(t2679: f64, t969: f64, t825: f64, t1964: f64, t60: f64, t822: f64) -> (f64, f64, f64) {
    let t2680 = t969 * t2679;
    let t2681 = t825 * t2680;
    let t2683 = t1964 * t60;
    let t2684 = t822 * t2683;
    (t2681, t2683, t2684)
}
