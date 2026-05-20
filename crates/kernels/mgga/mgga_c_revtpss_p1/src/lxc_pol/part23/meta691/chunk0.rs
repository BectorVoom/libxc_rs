//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2434/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2434<F: Float>(t1358: F, t588: F, t9647: F, t4086: F, t9646: F, t1399: F, t22: F, t555: F, t1429: F, t39501: F, t1419: F, t5744: F) -> (F, F, F, F, F) {
    let t46388 = F::cast_from(0.15709759505761725819e-2_f64) * t9647 * t1358 * t588;
    let t46389 = t9646 * t4086;
    let t46392 = t46389 * t555 * t22 * t1399;
    let t46412 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1429;
    let t46456 = t5744 * t1419;
    (t46388, t46389, t46392, t46412, t46456)
}
