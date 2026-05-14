//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 883/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk883<F: Float>(t3217: F, t9009: F, t2586: F, t3205: F, t1170: F, t1156: F, t3213: F, t3212: F, t1128: F, t3194: F, t3192: F, t1152: F, t7274: F, t1150: F, t429: F, t438: F, t8905: F) -> (F, F, F, F, F, F) {
    let t9010 = t3217 * t9009;
    let t9012 = t2586 * t3205;
    let t9013 = t1170 * t9012;
    let t9015 = t1156 * t3213;
    let t9016 = t3212 * t9015;
    let t9018 = t1128 * t3194;
    let t9019 = t3192 * t9018;
    let t9021 = t7274 * t1152;
    let t9022 = t1150 * t9021;
    let t9025 = t429 * t8905 * t438;
    (t9010, t9013, t9016, t9019, t9022, t9025)
}
