//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 692/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk692(t1061: f64, t1095: f64, t7476: f64, t1980: f64, t1988: f64, t2109: f64, t368: f64, t7380: f64, t355: f64, t372: f64, t1083: f64, t2095: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7478 = t7476 * t1095 * t1061;
    let t7479 = t1980 * t7478;
    let t7481 = t1988 * t2109;
    let t7483 = t368 * t1061;
    let t7484 = t7380 * t7483;
    let t7486 = t355 * t372;
    let t7487 = t1083 * t7486;
    let t7488 = t2095 * t7487;
    (t7478, t7479, t7481, t7483, t7484, t7486, t7487, t7488)
}
