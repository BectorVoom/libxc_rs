//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1213/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1213(t15260: f64, t15499: f64, t15528: f64, t15577: f64, t1268: f64, t11178: f64, t1240: f64, t13327: f64, t13332: f64, t13337: f64, t13340: f64, t13344: f64, t13348: f64, t1857: f64, t9557: f64, t9559: f64, t9563: f64) -> f64 {
    let t15579 = t15260 + t15499 + t15528 + t15577;
    let t15580 = t15579 * t1268;
    let t15585 = 0.17024129629629629629e-1_f64 * t13327 - 0.15476481481481481481e-2_f64 * t13332 - 0.23214722222222222222e-2_f64 * t13337 - 0.77382407407407407406e-3_f64 * t13340 + 0.61905925925925925926e-2_f64 * t13344 + 0.11349419753086419753e-1_f64 * t13348 - 0.15476481481481481481e-2_f64 * t9557 - 0.41270617283950617284e-2_f64 * t9559 - 0.51588271604938271604e-3_f64 * t9563 - 0.66725e-1_f64 * t1240 * t15580 - 0.66725e-1_f64 * t11178 * t1857;
    t15585
}
