//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1804/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1804(t23185: f64, t4283: f64, t81914: f64, t25300: f64, t81591: f64, t81633: f64, t25303: f64, t6579: f64, t23110: f64, t4292: f64, t25288: f64, t234: f64, t4265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87544 = t23185 * t81914 * t4283;
    let t87546 = t81591 * t25300;
    let t87559 = 0.25587863262083522346e0_f64 * t81633;
    let t87565 = t6579 * t25303;
    let t87581 = t23185 * t23110 * t4292;
    let t87583 = t81591 * t25288;
    let t87586 = t234 * t4265;
    (t87544, t87546, t87559, t87565, t87581, t87583, t87586)
}
