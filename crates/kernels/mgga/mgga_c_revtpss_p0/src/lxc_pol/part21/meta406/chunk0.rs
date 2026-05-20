//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1870/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1870<F: Float>(t11249: F, t13045: F, t13044: F, t1042: F, t13040: F, t3597: F, t13036: F, t3603: F, t13032: F, t3609: F, t1244: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
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
