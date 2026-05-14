//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 884/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk884<F: Float>(t9025: F, t914: F, t438: F, t449: F, t8905: F, t894: F, t1172: F, t7878: F, t1170: F, t1128: F, t3188: F, t3186: F, t2856: F, t3236: F, t3235: F, t1900: F, t553: F) -> (F, F, F, F, F, F, F, F) {
    let t9026 = t914 * t9025;
    let t9030 = t449 * t8905 * t438;
    let t9031 = t894 * t9030;
    let t9034 = t7878 * t1172;
    let t9035 = t1170 * t9034;
    let t9037 = t1128 * t3188;
    let t9038 = t3186 * t9037;
    let t9040 = t2856 * t3236;
    let t9041 = t3235 * t9040;
    let t9044 = t553 * t1900;
    (t9026, t9030, t9031, t9035, t9038, t9040, t9041, t9044)
}
