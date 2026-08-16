//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2274/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2274(t1268: f64, t86604: f64, t1873: f64, t55934: f64, t12725: f64, t6534: f64, t55962: f64, t19456: f64, t4072: f64, t649: f64, t26114: f64, t12813: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t90361 = 2.0_f64 * t1268 * t86604;
    let t90363 = 4.0_f64 * t55934 * t1873;
    let t90365 = 4.0_f64 * t12725 * t6534;
    let t90367 = 2.0_f64 * t55962 * t1873;
    let t90369 = 4.0_f64 * t19456 * t6534;
    let t90370 = t649 * t4072;
    let t90372 = 4.0_f64 * t90370 * t1873;
    let t90374 = 4.0_f64 * t26114 * t6534;
    let t90375 = t88 * t12813;
    (t90361, t90363, t90365, t90367, t90369, t90370, t90372, t90374, t90375)
}
