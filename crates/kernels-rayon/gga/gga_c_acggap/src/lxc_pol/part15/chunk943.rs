//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 943/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk943(t2131: f64, t2147: f64, t463: f64, t8103: f64, t2176: f64, t3889: f64, t8111: f64, t872: f64, t2217: f64, t323: f64, t851: f64, t633: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t33053 = t2131 * t2147 * t8103 * t463;
    let t33063 = t2176 * t3889;
    let t33065 = t8111 * t872;
    let t33080 = t851 * t2217 * t323;
    let t33092 = t848 * t633;
    (t33053, t33063, t33065, t33080, t33092)
}
