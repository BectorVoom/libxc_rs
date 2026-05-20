//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1384/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1384<F: Float>(t225: F, t46361: F, t3896: F, t39515: F, t1362: F, t1363: F, t39497: F, t1358: F, t588: F, t9647: F, t4086: F, t9646: F) -> (F, F, F, F, F) {
    let t46362 = t225 * t46361;
    let t46368 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t3896;
    let t46385 = F::cast_from(0.10118827226026589797e0_f64) * t1362 * t1363 * t39497;
    let t46388 = F::cast_from(0.15709759505761725819e-2_f64) * t9647 * t1358 * t588;
    let t46389 = t9646 * t4086;
    (t46362, t46368, t46385, t46388, t46389)
}
