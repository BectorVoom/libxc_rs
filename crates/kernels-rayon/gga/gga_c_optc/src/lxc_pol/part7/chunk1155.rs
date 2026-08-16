//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1155/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1155(t2468: f64, t7337: f64, t2471: f64, t2528: f64, t23789: f64, t7504: f64, t845: f64, t2253: f64, t7188: f64, t23559: f64, t24003: f64, t24006: f64, t24008: f64, t24014: f64, t24017: f64, t2549: f64, t7175: f64, t7325: f64, t8280: f64, t8387: f64, t914: f64, t999: f64) -> (f64, f64, f64, f64) {
    let t24019 = 0.35089340384731224426e1_f64 * t7337 * t2468;
    let t24021 = 1.0_f64 / t2471 / t2528;
    let t24025 = 0.12304676425209353917e5_f64 * t845 * t24021 * t23789 * t7504;
    let t24026 = t7188 * t2253;
    let t24032 = -2.0_f64 / 9.0_f64 * t24003 - 8.0_f64 / 27.0_f64 * t24006 - 4.0_f64 / 3.0_f64 * t24008 + 8.0_f64 * t999 * t914 * t2549 * t23559 + 176.0_f64 / 27.0_f64 * t24014 - t24017 - t24019 + t24025 - 200.0_f64 / 3.0_f64 * t24026 - 400.0_f64 / 27.0_f64 * t8280 * t7175 - 400.0_f64 / 9.0_f64 * t7325 * t8387;
    (t24019, t24021, t24025, t24032)
}
