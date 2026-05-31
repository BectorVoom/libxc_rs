//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1188/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1188<F: Float>(t10940: F, t12033: F, t10634: F, t12098: F, t3262: F, t3465: F, t40383: F, t11336: F, t37327: F, t40297: F, t3719: F, t792: F) -> (F, F, F, F, F) {
    let t41179 = t10940 * t12033 / F::cast_from(4.0_f64);
    let t41182 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t12098 * t10634;
    let t41185 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t3465 * t40383;
    let t41188 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37327 * t11336 * t40297;
    let t41189 = t3719 * t792;
    (t41179, t41182, t41185, t41188, t41189)
}
