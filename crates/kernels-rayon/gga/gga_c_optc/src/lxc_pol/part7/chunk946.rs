//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 946/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk946(t3192: f64, t9018: f64, t1152: f64, t7274: f64, t1150: f64, t429: f64, t438: f64, t8905: f64, t914: f64, t449: f64, t894: f64, t1172: f64, t7878: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9019 = t3192 * t9018;
    let t9021 = t7274 * t1152;
    let t9022 = t1150 * t9021;
    let t9025 = t429 * t8905 * t438;
    let t9026 = t914 * t9025;
    let t9030 = t449 * t8905 * t438;
    let t9031 = t894 * t9030;
    let t9034 = t7878 * t1172;
    (t9019, t9022, t9025, t9026, t9030, t9031, t9034)
}
