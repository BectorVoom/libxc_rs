//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1036/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1036(t1268: f64, t12724: f64, t12725: f64, t12728: f64, t12734: f64, t12739: f64, t12813: f64, t1458: f64, t2314: f64, t2363: f64, t4028: f64, t4072: f64, t5113: f64, t671: f64, t9348: f64) -> f64 {
    let t12816 = 2.0_f64 * t1268 * t12813 + 4.0_f64 * t12725 * t671 + 4.0_f64 * t12734 * t1458 + 2.0_f64 * t12739 * t1458 + 2.0_f64 * t1458 * t9348 + 4.0_f64 * t2314 * t4072 + 2.0_f64 * t2363 * t4028 + 4.0_f64 * t4072 * t5113 + t12724 + 2.0_f64 * t12728;
    t12816
}
