//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 729/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk729(t6531: f64, t6565: f64, t530: f64, t186: f64, t185: f64, t5215: f64, t786: f64, t1982: f64, t808: f64, t2100: f64, t795: f64, t2407: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6566 = t6531 + t6565;
    let t6567 = t530 * t6566;
    let t6568 = t186 * t6567;
    let t6570 = 2.0_f64 / 15.0_f64 * t185 * t6568;
    let t6572 = 8.0_f64 / 15.0_f64 * t5215 * t786;
    let t6574 = 4.0_f64 / 15.0_f64 * t1982 * t808;
    let t6576 = 4.0_f64 / 15.0_f64 * t795 * t2100;
    let t6578 = 4.0_f64 / 15.0_f64 * t2407 * t544;
    (t6566, t6567, t6568, t6570, t6572, t6574, t6576, t6578)
}
