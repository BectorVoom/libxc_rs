//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2010/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2010(t102344: f64, t1458: f64, t19534: f64, t2039: f64, t2314: f64, t23938: f64, t26114: f64, t26117: f64, t26977: f64, t27170: f64, t27188: f64, t28007: f64, t28951: f64, t33234: f64, t4072: f64, t5113: f64, t5493: f64, t55943: f64, t7042: f64, t7056: f64, t7676: f64, t7801: f64, t96657: f64) -> f64 {
    let t102432 = 4.0_f64 * t102344 * t1458 + 2.0_f64 * t19534 * t7042 + 2.0_f64 * t2039 * t55943 + 2.0_f64 * t2039 * t96657 + 2.0_f64 * t2314 * t28951 + 2.0_f64 * t23938 * t5493 + 4.0_f64 * t26114 * t7801 + 4.0_f64 * t26117 * t7801 + 2.0_f64 * t26977 * t5493 + 4.0_f64 * t27170 * t7676 + 4.0_f64 * t27188 * t4072 + 2.0_f64 * t28007 * t7056 + 2.0_f64 * t28951 * t5113 + 4.0_f64 * t33234 * t4072;
    t102432
}
