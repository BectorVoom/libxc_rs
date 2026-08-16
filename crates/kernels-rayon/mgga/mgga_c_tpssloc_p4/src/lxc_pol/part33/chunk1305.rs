//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1305/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1305(t28821: f64, t7756: f64, t28823: f64, t7685: f64, t28835: f64, t1983: f64, t7687: f64, t97817: f64, t7688: f64, t28860: f64, t19451: f64, t7468: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105167 = 3.0_f64 * t28821 * t7756;
    let t105169 = 6.0_f64 * t7685 * t28823;
    let t105171 = 9.0_f64 * t7685 * t28835;
    let t105175 = 9.0_f64 * t1983 * t97817 * t7687;
    let t105177 = 9.0_f64 * t28821 * t7688;
    let t105179 = 3.0_f64 * t7685 * t28860;
    let t105181 = 6.0_f64 * t19451 * t7468;
    (t105167, t105169, t105171, t105175, t105177, t105179, t105181)
}
