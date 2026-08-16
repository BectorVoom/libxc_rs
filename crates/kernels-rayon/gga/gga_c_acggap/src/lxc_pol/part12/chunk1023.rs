//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1023/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1023(t2304: f64, t7610: f64, t1988: f64, t8561: f64, t8566: f64, t2001: f64, t4754: f64, t4759: f64, t4764: f64, t4456: f64, t8511: f64, t1165: f64, t4665: f64, t7564: f64, t8600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    let t34221 = t1988 * t8566;
    let t34223 = t2001 * t4754;
    let t34225 = t2001 * t4759;
    let t34227 = t2001 * t4764;
    let t34229 = t8511 * t4456;
    let t34233 = t7564 * t1165 * t8600 * t4665;
    (t34215, t34217, t34221, t34223, t34225, t34227, t34229, t34233)
}
