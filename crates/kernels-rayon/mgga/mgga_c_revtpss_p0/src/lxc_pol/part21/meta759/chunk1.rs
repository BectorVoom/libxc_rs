//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2679/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2679(t14230: f64, t2782: f64, t46456: f64, t1385: f64, t14066: f64, t14155: f64, t1432: f64, t2470: f64, t3999: f64, t5710: f64, t1892: f64, t4056: f64) -> (f64, f64, f64, f64, f64) {
    let t49263 = t2782 * t46456 * t14230;
    let t49268 = t1385 * t14066;
    let t49273 = t1432 * t14155 * t2470;
    let t49274 = 0.39029762157531132076e-1_f64 * t49273;
    let t49276 = t3999 * t5710;
    let t49280 = t1892 * t4056;
    (t49263, t49268, t49274, t49276, t49280)
}
