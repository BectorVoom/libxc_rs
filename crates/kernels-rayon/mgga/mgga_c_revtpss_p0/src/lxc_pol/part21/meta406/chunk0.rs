//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1870/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1870(t11249: f64, t13045: f64, t13044: f64, t1042: f64, t13040: f64, t3597: f64, t13036: f64, t3603: f64, t13032: f64, t3609: f64, t1244: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13046 = t11249 * t13045;
    let t13047 = t13044 * t13046;
    let t13048 = t1042 * t13047;
    let t13051 = t3597 * t13040;
    let t13052 = t13036 * t13051;
    let t13053 = t11249 * t3603;
    let t13054 = t13044 * t13053;
    let t13055 = t1042 * t13054;
    let t13058 = t13032 * t3609;
    let t13061 = t1244 * t13040;
    let t13062 = t13036 * t13061;
    let t13063 = t11249 * t471;
    (t13046, t13047, t13048, t13051, t13052, t13053, t13054, t13055, t13058, t13061, t13062, t13063)
}
