//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 780/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk780(t1873: f64, t4028: f64, t1458: f64, t88: f64, t1268: f64, t7467: f64, t6517: f64, t7451: f64, t1778: f64, t191: f64, t192: f64) -> (f64, f64, f64) {
    let t7675 = 2.0_f64 * t4028 * t1873;
    let t7676 = t88 * t1458;
    let t7678 = 2.0_f64 * t7676 * t1873;
    let t7680 = 2.0_f64 * t1268 * t7467;
    let t7681 = 2.0_f64 * t1458 * t6517 + t7451 + t7675 + t7678 + t7680;
    let t7684 = t1778 * t191;
    let t7685 = t7684 * t192;
    (t7676, t7681, t7685)
}
