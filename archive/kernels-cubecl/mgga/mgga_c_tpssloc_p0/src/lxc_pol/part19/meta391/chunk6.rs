//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1477/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1477<F: Float>(t1174: F, t3556: F, t698: F, t11844: F, t135: F, t11849: F, t11662: F, t11665: F, t11678: F, t11680: F, t11688: F, t11692: F, t11694: F, t11709: F, t3248: F, t3252: F, t3506: F, t3509: F, t3516: F, t3560: F, t3578: F, t39103: F, t44774: F, t44879: F, t45134: F, t45148: F, t45162: F, t45167: F, t45169: F, t45171: F, t4582: F, t484: F, t488: F, t4978: F, t68: F, t974: F) -> F {
    let t45178 = t1174 * t698 * t3556;
    let t45181 = t1174 * t135 * t11844;
    let t45184 = t1174 * t135 * t11849;
    let t45186 = t45134 * t11694 / F::cast_from(384.0_f64) + t11692 * t3578 * t3516 * t3252 / F::cast_from(768.0_f64) + t44774 * t68 * t484 * t488 / F::cast_from(3072.0_f64) + t11709 * t11662 / F::cast_from(128.0_f64) - t45148 / F::cast_from(384.0_f64) + t3506 * t4582 * t44879 * t4978 / F::cast_from(384.0_f64) - t11678 * t3578 * t3509 * t3252 / F::cast_from(384.0_f64) - t11678 * t3578 * t3509 * t3248 / F::cast_from(192.0_f64) - t45162 * t11680 / F::cast_from(192.0_f64) - t11665 * t11688 / F::cast_from(192.0_f64) + t45167 / F::cast_from(384.0_f64) + t45169 / F::cast_from(192.0_f64) - t45171 / F::cast_from(384.0_f64) + t1174 * t974 * t3560 * t39103 / F::cast_from(72.0_f64) + t45178 / F::cast_from(108.0_f64) - t45181 / F::cast_from(216.0_f64) - t45184 / F::cast_from(36.0_f64);
    t45186
}
