//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1129/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1129(t1448: f64, t448: f64, t1444: f64, t452: f64, t1413: f64, t1449: f64, t1450: f64, t1466: f64, t16047: f64, t19417: f64, t19461: f64, t2484: f64, t2507: f64, t2510: f64, t2513: f64, t2528: f64, t430: f64, t459: f64, t4769: f64, t4772: f64, t4828: f64, t4829: f64, t4856: f64, t6634: f64, t6639: f64, t6642: f64, t6645: f64, t6692: f64, t987: f64, t995: f64) -> f64 {
    let t19467 = t448 * t1448;
    let t19470 = t1444 * t452;
    let t19495 = -0.99375e-1_f64 * t1413 * t6692 * t459 - 0.99375e-1_f64 * t1413 * t2507 * t1466 - 0.33125e-1_f64 * t1413 * t987 * t4856 + 0.165625e-1_f64 * t430 * (t19417 + t19461) - 0.99375e-1_f64 * t4769 * t2528 + 0.298125e0_f64 * t19467 * t6639 - 0.99375e-1_f64 * t19470 * t2484 - 0.19875e0_f64 * t6634 * t6642 - 0.99375e-1_f64 * t6634 * t6645 + 0.298125e0_f64 * t4772 * t2510 * t1466 + 0.298125e0_f64 * t4772 * t2507 * t1450 + 0.1490625e0_f64 * t1449 * t2528 * t1466 + 0.496875e-1_f64 * t1449 * t995 * t4856 + 0.99375e0_f64 * t16047 * t995 * t4829 - 0.59625e0_f64 * t4828 * t2513 * t1466;
    t19495
}
