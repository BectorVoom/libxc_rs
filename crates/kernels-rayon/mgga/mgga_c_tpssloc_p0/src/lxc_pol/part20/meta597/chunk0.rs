//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2177/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2177(t11496: f64, t3448: f64, t11502: f64, t1184: f64, t15418: f64, t11571: f64, t3447: f64, t3469: f64, t4899: f64, t11570: f64, t9288: f64, t3450: f64, t9258: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44517 = t3448 * t11496;
    let t44521 = t3448 * t11502;
    let t44525 = t15418 * t1184;
    let t44527 = t3447 * t44525 * t11571;
    let t44529 = t4899 * t3469;
    let t44536 = t11570 * t9288;
    let t44540 = t3450 * t9258;
    (t44517, t44521, t44525, t44527, t44529, t44536, t44540)
}
