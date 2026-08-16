//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1233/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233<F: Float>(t232: F, t41332: F, t2617: F, t9670: F, t831: F, t13254: F, t237: F, t249: F, t2618: F, t2623: F, t2645: F, t41123: F, t41130: F, t41132: F, t41134: F, t41139: F, t41231: F, t41237: F, t4178: F, t817: F, t819: F, t820: F, t9618: F, t9626: F, t9634: F, t9663: F, t9960: F) -> (F, F) {
    let t41333 = t41332 * t232;
    let t41340 = t2617 * t9670;
    let t41341 = t41340 * t831;
    let t41343 = -t4178 * t2645 * t9626 * t41123 / F::cast_from(64.0_f64) + t13254 * t9634 / F::cast_from(128.0_f64) - F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t41130 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t41132 + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t41134 + t41139 + t41231 * t237 * t249 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2623 * t9618 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t41237 - t2618 * t9960 / F::cast_from(768.0_f64) - t817 * t819 * t820 * t41333 / F::cast_from(3072.0_f64) - t2618 * t9663 / F::cast_from(768.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t41341;
    (t41333, t41343)
}
