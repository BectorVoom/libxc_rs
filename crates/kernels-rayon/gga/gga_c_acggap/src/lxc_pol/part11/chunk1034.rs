//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1034/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1034(t1988: f64, t8566: f64, t2001: f64, t4754: f64, t4759: f64, t4764: f64, t4456: f64, t8511: f64, t1165: f64, t4665: f64, t7564: f64, t8600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34221 = t1988 * t8566;
    let t34222 = 0.62896184579208304136e-3_f64 * t34221;
    let t34223 = t2001 * t4754;
    let t34225 = t2001 * t4759;
    let t34227 = t2001 * t4764;
    let t34229 = t8511 * t4456;
    let t34233 = t7564 * t1165 * t8600 * t4665;
    (t34222, t34223, t34225, t34227, t34229, t34233)
}
