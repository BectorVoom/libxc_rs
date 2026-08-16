//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 874/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk874(t2040: f64, t33211: f64, t7467: f64, t89: f64, t7796: f64, t8526: f64, t1845: f64, t2018: f64, t26558: f64, t26161: f64, t4028: f64, t8533: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33213 = 2.0_f64 * t33211 * t2040;
    let t33214 = t89 * t7467;
    let t33216 = 2.0_f64 * t33214 * t2040;
    let t33218 = 2.0_f64 * t8526 * t7796;
    let t33221 = t2018 * t1845;
    let t33222 = t26558 * t33221;
    let t33224 = 2.0_f64 * t26161 * t33222;
    let t33227 = 2.0_f64 * t4028 * t8533;
    (t33213, t33214, t33216, t33218, t33221, t33222, t33224, t33227)
}
