//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 544/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk544(t210: f64, t214: f64, t4119: f64, t2562: f64, t2564: f64, t2569: f64, t2579: f64, t2590: f64, t4124: f64, t4127: f64, t4130: f64, t4135: f64, t787: f64) -> f64 {
    let t4138 = t210 * t214 * t4119;
    let t4142 = t2562 + 0.38888888888888888888e-2_f64 * t2564 + t2569 + 0.38888888888888888887e-2_f64 * t4124 + 0.49999999999999999998e-2_f64 * t4127 * t4130 + 0.8333333333333333333e-3_f64 * t4135 - 0.16666666666666666666e-2_f64 * t787 * t4138 + 0.83333333333333333332e-3_f64 * t2579 - t2590;
    t4142
}
