//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1700/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1700<F: Float>(t33: F, t516: F, t9615: F, t3842: F, t3351: F, t1348: F, t3881: F, t43744: F, t9357: F, t9617: F, t9620: F, t46325: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t46328 = F::cast_from(1.0_f64) / t516 / t9615 / t33;
    let t46329 = t3842 * t3842;
    let t46335 = t3351 * t3351;
    let t46343 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46328 * t46329 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9617 * t3842 * t3351 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3881 * t46335 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9620 * t9357 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1348 * t43744);
    let t46345 = t46325 / F::cast_from(2.0_f64) + t46343 / F::cast_from(2.0_f64);
    (t46329, t46335, t46345)
}
