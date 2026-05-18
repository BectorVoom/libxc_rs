//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 946/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk946<F: Float>(t3192: F, t9018: F, t1152: F, t7274: F, t1150: F, t429: F, t438: F, t8905: F, t914: F, t449: F, t894: F, t1172: F, t7878: F) -> (F, F, F, F, F, F, F) {
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
