//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 305/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk305(t442: f64, t459: f64, t1056: f64, t1422: f64, t306: f64, t1175: f64, t457: f64, t425: f64, t458: f64, t1364: f64, t1216: f64, t1419: f64, t1421: f64, t338: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1423 = t459 * t442;
    let t1425 = t1422 * t1423 * t1056;
    let t1428 = t306 * t459;
    let t1429 = t1428 * t1175;
    let t1430 = t457 * t1429;
    let t1433 = t458 * t425;
    let t1434 = t1433 * t1364;
    let t1435 = t457 * t1434;
    let t1440 = t1419 + 0.65704296666666666667e-3_f64 * t1421 * t1425 + 0.1478346675e-2_f64 * t456 * t1430 - 0.98556445e-3_f64 * t456 * t1435 - 4.0_f64 * t338 * t1216;
    (t1423, t1425, t1428, t1429, t1430, t1433, t1434, t1435, t1440)
}
