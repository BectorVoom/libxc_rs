//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1704/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1704<F: Float>(t1358: F, t588: F, t9647: F, t4086: F, t9646: F, t1399: F, t22: F, t555: F, t9890: F, t10040: F, t2435: F, t10039: F, t2439: F, t2777: F) -> (F, F, F, F, F) {
    let t46388 = F::cast_from(0.15709759505761725819e-2_f64) * t9647 * t1358 * t588;
    let t46389 = t9646 * t4086;
    let t46392 = t46389 * t555 * t22 * t1399;
    let t46394 = t555 * t9890;
    let t46398 = t2435 * t10040;
    let t46401 = t2439 * t2777 * t10039;
    (t46388, t46392, t46394, t46398, t46401)
}
