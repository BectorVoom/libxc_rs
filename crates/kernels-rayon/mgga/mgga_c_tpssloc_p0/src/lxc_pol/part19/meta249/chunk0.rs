//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 991/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk991(t11496: f64, t457: f64, t460: f64, t974: f64, t1184: f64, t3475: f64, t3469: f64, t4934: f64, t135: f64, t3477: f64, t1174: f64, t11153: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11498 = t457 * t11496 * t460;
    let t11499 = t974 * t11498;
    let t11502 = t3475 * t1184;
    let t11504 = t457 * t11502 * t460;
    let t11505 = t974 * t11504;
    let t11509 = t3469 * t1184 * t460;
    let t11510 = t4934 * t11509;
    let t11513 = t135 * t3477;
    let t11514 = t1174 * t11513;
    let t11516 = t461 * t11153;
    (t11498, t11499, t11502, t11504, t11505, t11509, t11510, t11514, t11516)
}
