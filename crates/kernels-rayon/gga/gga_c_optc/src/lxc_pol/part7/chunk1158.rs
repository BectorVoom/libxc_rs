//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1158/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1158(t23: f64, t287: f64, t8291: f64, t8294: f64, t8297: f64, t2474: f64, t2534: f64, t845: f64, t279: f64, t5714: f64, t1001: f64, t3902: f64, t999: f64) -> (f64, f64, f64, f64) {
    let t24072 = t8291 * t8294 * t23 * t287 * t8297;
    let t24076 = 0.21053604230838734656e2_f64 * t845 * t2474 * t2534;
    let t24088 = 1.0_f64 / t279 / t5714;
    let t24094 = t999 * t3902 * t1001;
    (t24072, t24076, t24088, t24094)
}
