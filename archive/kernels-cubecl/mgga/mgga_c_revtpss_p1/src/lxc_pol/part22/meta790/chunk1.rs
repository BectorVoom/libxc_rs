//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2882/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2882<F: Float>(t1362: F, t1363: F, t39497: F, t1358: F, t588: F, t9647: F, t4086: F, t9646: F, t1399: F, t22: F, t555: F, t10040: F, t2435: F) -> (F, F, F, F, F) {
    let t46385 = F::cast_from(0.10118827226026589797e0_f64) * t1362 * t1363 * t39497;
    let t46388 = F::cast_from(0.15709759505761725819e-2_f64) * t9647 * t1358 * t588;
    let t46389 = t9646 * t4086;
    let t46392 = t46389 * t555 * t22 * t1399;
    let t46398 = t2435 * t10040;
    (t46385, t46388, t46389, t46392, t46398)
}
