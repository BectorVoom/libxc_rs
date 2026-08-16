//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1076/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1076(t2176: f64, t5368: f64, t1620: f64, t8331: f64, t33796: f64, t8313: f64, t33799: f64, t8310: f64, t38086: f64, t4210: f64, t7942: f64, t524: f64, t9427: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38361 = t2176 * t5368;
    let t38370 = 0.26341796731742046394e1_f64 * t8331 * t1620;
    let t38377 = 0.17347256376410398924e1_f64 * t33796 * t8313;
    let t38379 = 0.17347256376410398924e1_f64 * t33799 * t8310;
    let t38382 = 0.17347256376410398924e1_f64 * t7942 * t38086 * t4210;
    let t38383 = t9427 * t524;
    (t38361, t38370, t38377, t38379, t38382, t38383)
}
