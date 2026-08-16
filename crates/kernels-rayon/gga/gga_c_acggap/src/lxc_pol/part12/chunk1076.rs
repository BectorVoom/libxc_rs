//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1076/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1076(t142: f64, t5187: f64, t8888: f64, t507: f64, t7436: f64, t961: f64, t1165: f64, t20138: f64, t604: f64, t7413: f64, t1992: f64, t30127: f64, t7842: f64, t8791: f64) -> (f64, f64, f64, f64) {
    let t35154 = t8888 * t142 * t5187;
    let t35157 = t7436 * t507 * t961;
    let t35172 = t7413 * t1165 * t604 * t20138;
    let t35176 = t30127 * t7842 * t1992 * t8791;
    (t35154, t35157, t35172, t35176)
}
