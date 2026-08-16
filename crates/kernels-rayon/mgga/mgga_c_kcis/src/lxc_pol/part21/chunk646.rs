//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 646/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk646(t1788: f64, t3393: f64, t148: f64, t41: f64, t85: f64, t2840: f64, t339: f64, t4567: f64, t1154: f64, t1646: f64, t3405: f64, t1018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5130 = t3393 * t1788;
    let t5133 = t85 * t148 * t41;
    let t5134 = t2840 * t339;
    let t5135 = t5134 * t4567;
    let t5139 = t1154 * t3405 * t1646;
    let t5142 = t1018 * t339;
    (t5130, t5133, t5134, t5135, t5139, t5142)
}
