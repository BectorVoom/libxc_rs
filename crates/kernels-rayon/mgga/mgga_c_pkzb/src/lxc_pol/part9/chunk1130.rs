//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1130/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1130(t1424: f64, t454: f64, t34: f64, t4794: f64, t38: f64, t4810: f64, t19378: f64, t19381: f64, t19384: f64, t19387: f64, t19390: f64, t19397: f64, t19400: f64, t19403: f64, t19410: f64, t19435: f64, t19439: f64, t6655: f64, t6659: f64, t6668: f64, t6723: f64, t6738: f64) -> f64 {
    let t19520 = t454 * t1424;
    let t19523 = t34 * t4794;
    let t19530 = t38 * t4810;
    let t19539 = 50.0_f64 / 27.0_f64 * t454 * t6655 + 25.0_f64 * t454 * t6668 + 40.0_f64 / 81.0_f64 * t34 * t19435 + 10.0_f64 / 3.0_f64 * t34 * t19439 - 10.0_f64 * t6723 * t19387 + 10.0_f64 * t6738 * t19390 - 100.0_f64 / 9.0_f64 * t19520 * t6659 - 10.0_f64 / 9.0_f64 * t19523 * t19397 - 10.0_f64 / 9.0_f64 * t19523 * t19400 + 10.0_f64 / 3.0_f64 * t6723 * t19403 - 10.0_f64 / 9.0_f64 * t19530 * t19378 + 10.0_f64 / 9.0_f64 * t19530 * t19381 - 10.0_f64 / 3.0_f64 * t6738 * t19384 - 10.0_f64 * t34 * t19410;
    t19539
}
