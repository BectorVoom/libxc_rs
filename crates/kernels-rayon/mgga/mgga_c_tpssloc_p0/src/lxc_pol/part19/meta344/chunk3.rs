//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1233/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233(t232: f64, t41332: f64, t2617: f64, t9670: f64, t831: f64, t13254: f64, t237: f64, t249: f64, t2618: f64, t2623: f64, t2645: f64, t41123: f64, t41130: f64, t41132: f64, t41134: f64, t41139: f64, t41231: f64, t41237: f64, t4178: f64, t817: f64, t819: f64, t820: f64, t9618: f64, t9626: f64, t9634: f64, t9663: f64, t9960: f64) -> (f64, f64) {
    let t41333 = t41332 * t232;
    let t41340 = t2617 * t9670;
    let t41341 = t41340 * t831;
    let t41343 = -t4178 * t2645 * t9626 * t41123 / 64.0_f64 + t13254 * t9634 / 128.0_f64 - 595.0_f64 / 2592.0_f64 * t41130 - 7.0_f64 / 1152.0_f64 * t41132 + 119.0_f64 / 2304.0_f64 * t41134 + t41139 + t41231 * t237 * t249 / 3072.0_f64 + 5.0_f64 / 64.0_f64 * t2623 * t9618 + 7.0_f64 / 1152.0_f64 * t41237 - t2618 * t9960 / 768.0_f64 - t817 * t819 * t820 * t41333 / 3072.0_f64 - t2618 * t9663 / 768.0_f64 - 119.0_f64 / 1152.0_f64 * t41341;
    (t41333, t41343)
}
