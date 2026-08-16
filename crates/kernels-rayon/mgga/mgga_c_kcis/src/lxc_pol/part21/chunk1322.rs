//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1322/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1322(t13161: f64, t2842: f64, t7718: f64, t13133: f64, t9370: f64, t13200: f64, t42530: f64, t1020: f64, t4792: f64, t92917: f64, t13256: f64, t26760: f64) -> (f64, f64, f64, f64, f64) {
    let t96318 = t2842 * t7718 * t13161;
    let t96321 = t9370 * t7718 * t13133;
    let t96324 = t42530 * t7718 * t13200;
    let t96327 = t1020 * t92917 * t4792;
    let t96330 = t1020 * t26760 * t13256;
    (t96318, t96321, t96324, t96327, t96330)
}
