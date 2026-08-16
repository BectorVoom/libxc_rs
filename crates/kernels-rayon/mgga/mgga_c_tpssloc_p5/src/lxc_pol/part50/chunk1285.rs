//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1285/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1285(t112516: f64, t112518: f64, t114439: f64, t114441: f64, t118373: f64, t120758: f64, t120762: f64, t120767: f64, t1404: f64, t1858: f64, t2029: f64, t26510: f64, t3: f64, t31254: f64, t33165: f64, t5381: f64, t580: f64, t8497: f64) -> f64 {
    let t120771 = t120758 * t3 * t580 + t1404 * t33165 + t1858 * t31254 + 2.0_f64 * t2029 * t26510 + t5381 * t8497 + t112516 + t112518 + 2.0_f64 * t114439 + 2.0_f64 * t114441 + t118373 + t120762 + 2.0_f64 * t120767;
    t120771
}
