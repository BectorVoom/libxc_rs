//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 897/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk897(t448: f64, t452: f64, t1450: f64, t987: f64, t2507: f64, t459: f64, t1466: f64, t1425: f64, t4794: f64, t973: f64, t1424: f64, t7: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6634 = t448 * t452;
    let t6639 = t987 * t1450;
    let t6642 = t2507 * t459;
    let t6645 = t987 * t1466;
    let t6655 = t4794 * t973 * t1425;
    let t6658 = t7 * t1424;
    (t6634, t6639, t6642, t6645, t6655, t6658)
}
