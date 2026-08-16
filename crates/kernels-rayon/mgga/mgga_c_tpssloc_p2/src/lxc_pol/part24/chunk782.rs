//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 782/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk782(t1958: f64, t3216: f64, t202: f64, t6665: f64, t1877: f64, t1915: f64, t193: f64, t2522: f64, t6670: f64, t776: f64, t868: f64, t870: f64) -> (f64, f64) {
    let t6822 = t1958 * t3216;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + 3.0_f64 * t1915 * t2522 * t776 + t193 * t6829 * t870;
    (t6822, t6834)
}
