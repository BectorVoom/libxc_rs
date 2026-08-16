//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1171/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1171(t20953: f64, t6614: f64, t20978: f64, t23146: f64, t20861: f64, t2628: f64, t6605: f64, t6552: f64, t7479: f64, t98133: f64, t1880: f64, t21013: f64, t214: f64, t225: f64, t258: f64) -> (f64, f64, f64, f64, f64) {
    let t105406 = t6614 * t20953;
    let t105412 = t23146 * t20978;
    let t105415 = t6605 * t2628 * t20861;
    let t105423 = t6552 * t98133 * t7479;
    let t105428 = t1880 * t214 * t21013 * t225 * t258;
    (t105406, t105412, t105415, t105423, t105428)
}
