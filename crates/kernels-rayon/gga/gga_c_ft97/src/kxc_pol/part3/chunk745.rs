//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 745/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk745(t15664: f64, t1594: f64, t3056: f64, t930: f64, t428: f64, t4467: f64, t374: f64, t15657: f64, t1631: f64, t15630: f64, t534: f64, t383: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15665 = t1594 * t15664;
    let t15668 = t930 * t3056;
    let t15669 = t1594 * t15668;
    let t15673 = t4467 * t428;
    let t15674 = t374 * t15673;
    let t15677 = t1631 * t15657;
    let t15680 = t534 * t15630;
    let t15681 = t77 * t383;
    (t15665, t15668, t15669, t15674, t15677, t15680, t15681)
}
