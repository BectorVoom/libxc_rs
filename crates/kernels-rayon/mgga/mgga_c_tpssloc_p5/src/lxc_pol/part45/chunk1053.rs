//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1053/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1053(t1873: f64, t84078: f64, t94165: f64, t24462: f64, t6534: f64, t114472: f64, t114483: f64, t114489: f64, t114494: f64, t114500: f64, t115983: f64, t115984: f64, t115990: f64, t115995: f64, t115996: f64, t116000: f64, t2039: f64, t2319: f64, t23877: f64, t23880: f64, t24481: f64, t671: f64, t7056: f64, t84004: f64, t91803: f64) -> f64 {
    let t116004 = 0.135e2_f64 * t84078 * t1873;
    let t116006 = 27.0_f64 * t94165 * t1873;
    let t116008 = 27.0_f64 * t24462 * t6534;
    let t116011 = t114472 + t115983 + 27.0_f64 * t115984 * t2319 + 0.135e2_f64 * t84004 * t2039 + t115990 + t114483 + 27.0_f64 * t23880 * t24481 + t114489 + t115995 + t114494 + 27.0_f64 * t115996 * t671 + t116000 + t114500 + 27.0_f64 * t23877 * t7056 + t116004 + t116006 + t116008 + 27.0_f64 * t91803 * t2039;
    t116011
}
