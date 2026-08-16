//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1005/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1005(t1090: f64, t3509: f64, t3578: f64, t1216: f64, t3252: f64, t3248: f64, t11642: f64, t11644: f64, t11649: f64, t11652: f64, t11655: f64, t11662: f64, t11665: f64, t11670: f64, t11674: f64, t11678: f64, t1227: f64, t3496: f64, t3506: f64, t3536: f64, t3577: f64, t3580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11679 = t3509 * t1090;
    let t11680 = t3578 * t11679;
    let t11683 = t3252 * t1216;
    let t11684 = t3578 * t11683;
    let t11687 = t3248 * t1216;
    let t11688 = t3578 * t11687;
    let t11691 = t11642 / 1536.0_f64 - t11644 / 4608.0_f64 + t11649 - t11652 / 1536.0_f64 + 5.0_f64 / 2304.0_f64 * t1227 * t11655 + t3536 * t3496 / 1024.0_f64 + t3506 * t11662 / 512.0_f64 - t11665 * t3580 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t11670 - t3577 * t11674 / 1536.0_f64 - t11678 * t11680 / 768.0_f64 - t3577 * t11684 / 1536.0_f64 - t3577 * t11688 / 768.0_f64;
    (t11679, t11680, t11683, t11684, t11687, t11688, t11691)
}
