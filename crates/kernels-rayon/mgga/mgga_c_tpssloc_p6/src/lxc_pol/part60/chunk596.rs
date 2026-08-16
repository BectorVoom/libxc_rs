//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 596/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk596(t1649: f64, t1877: f64, t1915: f64, t2522: f64, t28: f64, t6670: f64, t7541: f64, t7650: f64, t7656: f64, t1873: f64, t4028: f64, t1458: f64, t88: f64) -> (f64, f64, f64) {
    let t7663 = 3.0_f64 / 2.0_f64 * t2522 * t7650 + t1877 * t7541 * t28 / 2.0_f64 - t1877 * t6670 * t7656 / 2.0_f64 + t1877 * t1915 * t1649 / 2.0_f64;
    let t7675 = 2.0_f64 * t4028 * t1873;
    let t7676 = t88 * t1458;
    (t7663, t7675, t7676)
}
