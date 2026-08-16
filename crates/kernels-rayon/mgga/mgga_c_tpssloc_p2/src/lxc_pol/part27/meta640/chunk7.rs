//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2170/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2170(t25303: f64, t6579: f64, t1509: f64, t6624: f64, t13456: f64, t1888: f64, t6646: f64, t13450: f64, t23110: f64, t23185: f64, t4292: f64, t25288: f64, t81591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87565 = t6579 * t25303;
    let t87566 = 0.76763589786250567036e-1_f64 * t87565;
    let t87567 = t6624 * t1509;
    let t87575 = t1888 * t6646 * t13456;
    let t87578 = t1888 * t6646 * t13450;
    let t87581 = t23185 * t23110 * t4292;
    let t87582 = 0.82246703342411321824e-2_f64 * t87581;
    let t87583 = t81591 * t25288;
    (t87566, t87567, t87575, t87578, t87582, t87583)
}
