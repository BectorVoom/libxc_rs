//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1697/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1697<F: Float>(t114: F, t46232: F, t10259: F, t10260: F, t10263: F, t10416: F, t10426: F, t118: F, t1310: F, t1312: F, t13207: F, t13216: F, t13435: F, t1453: F, t2322: F, t2331: F, t2371: F, t3813: F, t4254: F, t43735: F, t45923: F, t46125: F, t46126: F, t46129: F, t46137: F, t508: F, t5523: F, t569: F, t651: F, t670: F, t93: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t46233 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t46232);
    let t46250 = -F::cast_from(8.0_f64) * t651 * t13207 * t670 - F::cast_from(12.0_f64) * t651 * t3813 * t2371 + F::cast_from(4.0_f64) * t10426 * t1453 - F::cast_from(24.0_f64) * t10416 * t2331 - F::cast_from(24.0_f64) * t2322 * t10263 - t118 * (t43735 + t45923) + (F::cast_from(8.0_f64) * t10259 * t2322 + F::cast_from(8.0_f64) * t10259 * t5523 + F::cast_from(12.0_f64) * t10416 * t2371 + F::cast_from(2.0_f64) * t1312 * t46233 + F::cast_from(24.0_f64) * t13435 * t2371 + F::cast_from(8.0_f64) * t46126 * t670 + F::cast_from(6.0_f64) * t46137 * t93 + t46125 + F::cast_from(12.0_f64) * t46129) * t569 - t46125 * t508 - F::cast_from(24.0_f64) * t2322 * t13216 - F::cast_from(8.0_f64) * t2322 * t10260 - F::cast_from(8.0_f64) * t4254 * t10260 - F::cast_from(8.0_f64) * t651 * t1310 * t10259 - F::cast_from(24.0_f64) * t4254 * t13216;
    (t46233, t46250)
}
