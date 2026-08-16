//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3169/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3169(t11546: f64, t1174: f64, t1232: f64, t14744: f64, t14753: f64, t15569: f64, t15710: f64, t15764: f64, t1735: f64, t1743: f64, t18395: f64, t3447: f64, t3566: f64, t3577: f64, t3578: f64, t45119: f64, t488: f64, t52696: f64, t52995: f64, t53187: f64, t55716: f64, t6164: f64, t63372: f64, t65567: f64, t65581: f64, t65598: f64, t65600: f64, t65605: f64, t65607: f64) -> f64 {
    let t65610 = t65567 / 54.0_f64 - t15764 * t1743 * t488 / 288.0_f64 + 19.0_f64 / 1728.0_f64 * t3566 * t6164 * t488 - t3447 * t53187 * t55716 / 12.0_f64 + t3447 * t52995 * t55716 / 9.0_f64 - t65581 / 13824.0_f64 - t45119 * t3578 * t52696 * t18395 / 2304.0_f64 - t3577 * t3578 * t1735 * t14753 / 1152.0_f64 - t3577 * t3578 * t1735 * t14744 / 384.0_f64 + t15569 * t15710 / 108.0_f64 + 5.0_f64 / 5184.0_f64 * t65598 + t65600 / 1296.0_f64 - 7.0_f64 / 648.0_f64 * t1174 * t11546 * t63372 - t65605 / 6912.0_f64 - t65607 * t1232 / 2304.0_f64;
    t65610
}
