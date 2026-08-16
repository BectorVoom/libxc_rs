//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1218/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1218(t2553: f64, t28: f64, t2749: f64, t1081: f64, t868: f64, t2745: f64, t1877: f64, t1915: f64, t22959: f64, t23286: f64, t23290: f64, t23295: f64, t23781: f64, t23789: f64, t23792: f64, t2522: f64, t3231: f64, t4314: f64, t6666: f64, t6670: f64, t6841: f64, t6848: f64) -> (f64, f64, f64, f64, f64) {
    let t23796 = t28 * t2553;
    let t23807 = t28 * t2749;
    let t23810 = t1081 * t868;
    let t23813 = t28 * t2745;
    let t23820 = 3.0_f64 * t4314 * t1915 * t23781 + 3.0_f64 * t2522 * t6666 * t6841 - 3.0_f64 * t22959 * t23789 + 3.0_f64 * t2522 * t1915 * t23792 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t23796 + t1877 * t23286 * t28 / 2.0_f64 - t1877 * t23290 * t6848 + t1877 * t6666 * t1081 + t1877 * t23295 * t23807 - t1877 * t6670 * t23810 - t1877 * t6670 * t23813 / 2.0_f64 + t1877 * t1915 * t3231 / 2.0_f64;
    (t23796, t23807, t23810, t23813, t23820)
}
