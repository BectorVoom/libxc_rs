//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1477/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1477(t1174: f64, t3556: f64, t698: f64, t11844: f64, t135: f64, t11849: f64, t11662: f64, t11665: f64, t11678: f64, t11680: f64, t11688: f64, t11692: f64, t11694: f64, t11709: f64, t3248: f64, t3252: f64, t3506: f64, t3509: f64, t3516: f64, t3560: f64, t3578: f64, t39103: f64, t44774: f64, t44879: f64, t45134: f64, t45148: f64, t45162: f64, t45167: f64, t45169: f64, t45171: f64, t4582: f64, t484: f64, t488: f64, t4978: f64, t68: f64, t974: f64) -> f64 {
    let t45178 = t1174 * t698 * t3556;
    let t45181 = t1174 * t135 * t11844;
    let t45184 = t1174 * t135 * t11849;
    let t45186 = t45134 * t11694 / 384.0_f64 + t11692 * t3578 * t3516 * t3252 / 768.0_f64 + t44774 * t68 * t484 * t488 / 3072.0_f64 + t11709 * t11662 / 128.0_f64 - t45148 / 384.0_f64 + t3506 * t4582 * t44879 * t4978 / 384.0_f64 - t11678 * t3578 * t3509 * t3252 / 384.0_f64 - t11678 * t3578 * t3509 * t3248 / 192.0_f64 - t45162 * t11680 / 192.0_f64 - t11665 * t11688 / 192.0_f64 + t45167 / 384.0_f64 + t45169 / 192.0_f64 - t45171 / 384.0_f64 + t1174 * t974 * t3560 * t39103 / 72.0_f64 + t45178 / 108.0_f64 - t45181 / 216.0_f64 - t45184 / 36.0_f64;
    t45186
}
