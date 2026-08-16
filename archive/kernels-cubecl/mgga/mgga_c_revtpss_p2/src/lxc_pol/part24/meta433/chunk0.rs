//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1383/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1383<F: Float>(t3853: F, t3860: F, t30: F, t513: F, t9603: F, t33: F, t516: F, t9615: F, t39552: F, t562: F, t560: F, t9655: F) -> (F, F, F, F, F) {
    let t46302 = t3860 * t3853;
    let t46303 = F::cast_from(72.0_f64) * t46302;
    let t46310 = F::cast_from(1.0_f64) / t513 / t9603 / t30;
    let t46328 = F::cast_from(1.0_f64) / t516 / t9615 / t33;
    let t46359 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t562;
    let t46361 = F::cast_from(1.0_f64) / t9655 / t560;
    (t46303, t46310, t46328, t46359, t46361)
}
