//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1143/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1143(t35685: f64, t13299: f64, t33944: f64, t33945: f64, t13287: f64, t2297: f64, t31195: f64, t3169: f64, t15386: f64, t35340: f64, t2288: f64, t4210: f64) -> (f64, f64, f64, f64, f64) {
    let t35686 = 11.0_f64 / 48.0_f64 * t35685;
    let t35691 = t33944 * t13299 * t33945;
    let t35695 = t31195 * t13287 * t2297 * t3169;
    let t35698 = t31195 * t15386 * t35340;
    let t35700 = t2288 * t4210;
    (t35686, t35691, t35695, t35698, t35700)
}
