//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2578/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2578<F: Float>(t10061: F, t10069: F, t2782: F, t4086: F, t46407: F, t543: F, t4003: F, t46565: F, t5744: F, t10073: F, t10111: F, t1428: F, t588: F) -> (F, F, F, F, F) {
    let t47403 = t10069 * t10061;
    let t47407 = t2782 * t4086 * t46407 * t543;
    let t47411 = t2782 * t5744 * t46565 * t4003;
    let t47413 = t10073 * t10061;
    let t47417 = F::cast_from(0.15709759505761725819e-2_f64) * t10111 * t1428 * t588;
    (t47403, t47407, t47411, t47413, t47417)
}
