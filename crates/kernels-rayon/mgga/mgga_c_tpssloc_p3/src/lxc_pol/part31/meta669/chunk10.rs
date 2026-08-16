//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1987/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1987(t13042: f64, t13053: f64, t16804: f64, t2047: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t29055: f64, t29056: f64, t29080: f64, t7830: f64, t7842: f64, t855: f64, t865: f64, t87929: f64, t92966: f64, t92976: f64, t99033: f64, t99036: f64) -> f64 {
    let t101828 = -t92966 + 4.0_f64 * t2713 * t29080 - t87929 + t16804 * t2047 * t259 - 0.6579736267392905746e-1_f64 * t99033 + 0.3289868133696452873e-1_f64 * t99036 - t92976 + 4.0_f64 * t13053 * t7830 - 2.0_f64 * t13042 * t7842 + 2.0_f64 * t855 * t2718 * t29055 * t865 - t2597 * t29056;
    t101828
}
