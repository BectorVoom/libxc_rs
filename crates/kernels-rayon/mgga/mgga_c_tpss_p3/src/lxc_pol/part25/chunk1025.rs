//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1025/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1025(t2162: f64, t3664: f64, t3628: f64, t3629: f64, t14169: f64, t783: f64, t14174: f64, t2175: f64, t2177: f64, t3665: f64, t125: f64, t4706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14179 = t2162 * t3664;
    let t14181 = t3628 * t3629 * t14179;
    let t14185 = t3628 * t14169 * t783;
    let t14189 = t2175 * t14174 * t2177;
    let t14193 = t3628 * t3629 * t3665;
    let t14197 = t3628 * t14174 * t783;
    let t14200 = t125 * t4706;
    (t14179, t14181, t14185, t14189, t14193, t14197, t14200)
}
