//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1159/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1159(t2363: f64, t7262: f64, t2368: f64, t7263: f64, t1000: f64, t1002: f64, t1007: f64, t1008: f64, t10109: f64, t1015: f64, t10825: f64, t10826: f64, t23579: f64, t2360: f64, t23825: f64, t24072: f64, t24076: f64, t24088: f64, t24094: f64, t2551: f64, t4038: f64, t7180: f64, t7259: f64, t914: f64, t999: f64) -> f64 {
    let t24096 = t7262 * t2363;
    let t24099 = t7263 * t2368;
    let t24105 = 16000000.0_f64 / 729.0_f64 * t24072 - t24076 - 4.0_f64 * t10109 * t7180 + t999 * t914 * t1000 * t23579 / 6.0_f64 + 4.0_f64 / 3.0_f64 * t7263 * t2551 + 56.0_f64 / 27.0_f64 * t2360 * t7259 - 304700.0_f64 / 243.0_f64 * t1007 * t1008 * t24088 * t1015 + 20.0_f64 / 81.0_f64 * t24094 - 16.0_f64 / 3.0_f64 * t24096 * t1002 + 2.0_f64 / 3.0_f64 * t24099 + 28.0_f64 / 9.0_f64 * t4038 * t10825 * t10826 * t23825;
    t24105
}
