//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1035/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1035(t1181: f64, t4521: f64, t604: f64, t7426: f64, t1466: f64, t30644: f64, t13889: f64, t2068: f64, t2267: f64, t4516: f64, t7351: f64, t7564: f64) -> (f64, f64, f64, f64) {
    let t34237 = t7426 * t1181 * t604 * t4521;
    let t34239 = t30644 * t1466;
    let t34240 = 0.17149607247227894789e-2_f64 * t34239;
    let t34242 = t2068 * t13889 * t2267;
    let t34246 = t7564 * t1181 * t7351 * t4516;
    (t34237, t34240, t34242, t34246)
}
