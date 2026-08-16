//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1804/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1804(t25: f64, t2749: f64, t606: f64, t868: f64, t2745: f64, t1877: f64, t1915: f64, t2249: f64, t22951: f64, t22959: f64, t22961: f64, t22964: f64, t22968: f64, t23286: f64, t23290: f64, t23295: f64, t2522: f64, t4314: f64, t6542: f64, t6666: f64, t6670: f64, t6671: f64) -> (f64, f64, f64, f64) {
    let t23296 = t25 * t2749;
    let t23299 = t606 * t868;
    let t23302 = t25 * t2745;
    let t23309 = 3.0_f64 * t4314 * t1915 * t22951 + 3.0_f64 * t2522 * t6666 * t6542 - 3.0_f64 * t22959 * t22961 + 3.0_f64 * t2522 * t1915 * t22964 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t22968 + t1877 * t23286 * t25 / 2.0_f64 - t1877 * t23290 * t6671 + t1877 * t6666 * t606 + t1877 * t23295 * t23296 - t1877 * t6670 * t23299 - t1877 * t6670 * t23302 / 2.0_f64 + t1877 * t1915 * t2249 / 2.0_f64;
    (t23296, t23299, t23302, t23309)
}
