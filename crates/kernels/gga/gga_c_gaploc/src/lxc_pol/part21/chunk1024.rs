//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1024/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1024<F: Float>(t4379: F, t9573: F, t1402: F, t1429: F, t3162: F, t20237: F, t544: F, t9287: F, t1305: F, t2476: F, t9438: F, t9439: F, t6974: F, t9441: F, t7014: F, t9450: F) -> (F, F, F, F, F, F, F) {
    let t30265 = 0.59584149919750711116e-1 * t4379 * t9573;
    let t30288 = 0.17875244975925213335e0 * t1429 * t1402 * t3162;
    let t30292 = t544 * t20237;
    let t30294 = 0.29792074959875355558e-1 * t30292 * t9287;
    let t30297 = t2476 * t9438 * t9439 * t1305;
    let t30299 = t6974 * t9441;
    let t30305 = t7014 * t9450;
    (t30265, t30288, t30292, t30294, t30297, t30299, t30305)
}
