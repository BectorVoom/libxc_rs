//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1131/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1131(t1268: f64, t12725: f64, t1458: f64, t19456: f64, t2039: f64, t2314: f64, t23938: f64, t26114: f64, t26117: f64, t26967: f64, t26977: f64, t27170: f64, t27188: f64, t4028: f64, t4072: f64, t5113: f64, t671: f64, t7042: f64, t7056: f64, t7676: f64, t7801: f64) -> f64 {
    let t27215 = 2.0_f64 * t1268 * t27170 + 2.0_f64 * t12725 * t2039 + 2.0_f64 * t1458 * t23938 + 2.0_f64 * t1458 * t26977 + 2.0_f64 * t19456 * t2039 + 2.0_f64 * t2039 * t26114 + 2.0_f64 * t2039 * t26117 + 2.0_f64 * t2314 * t7801 + 2.0_f64 * t27188 * t671 + 2.0_f64 * t4028 * t7056 + 2.0_f64 * t4072 * t7042 + 2.0_f64 * t5113 * t7801 + 2.0_f64 * t7056 * t7676 + t26967;
    t27215
}
