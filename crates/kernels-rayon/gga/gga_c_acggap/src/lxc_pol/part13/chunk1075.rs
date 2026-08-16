//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1075/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1075(t30717: f64, t1998: f64, t4625: f64, t2001: f64, t5113: f64, t5118: f64, t1434: f64, t7736: f64, t1418: f64, t7614: f64, t1089: f64, t598: f64, t6337: f64, t7679: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34743 = 35.0_f64 / 108.0_f64 * t30717;
    let t34745 = t1998 * t4625;
    let t34746 = 0.17149607247227894789e-2_f64 * t34745;
    let t34747 = t2001 * t5113;
    let t34749 = t2001 * t5118;
    let t34751 = t7736 * t1434;
    let t34753 = t7614 * t1418;
    let t34754 = 0.32012600194825403606e-1_f64 * t34753;
    let t34757 = t598 * t1089 * t6337 * t7679;
    (t34743, t34746, t34747, t34749, t34751, t34754, t34757)
}
