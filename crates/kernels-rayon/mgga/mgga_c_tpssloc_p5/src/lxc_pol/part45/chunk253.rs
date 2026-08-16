//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 253/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk253(t1055: f64, t1065: f64, t1050: f64, t1052: f64, t388: f64, t991: f64, t390: f64) -> (f64, f64, f64) {
    let t1066 = t1055 * t1065;
    let t1068 = t1050 * t388 - t1052 * t1066 + t388 * t991;
    let t1070 = 1.0_f64 / t390;
    (t1066, t1068, t1070)
}
