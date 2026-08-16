//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1103/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1103(t1164: f64, t8853: f64, t2068: f64, t2069: f64, t31142: f64, t8884: f64, t2019: f64, t8887: f64, t8889: f64, t142: f64, t5183: f64, t7436: f64) -> (f64, f64, f64, f64) {
    let t35137 = t1164 * t8853;
    let t35139 = t2068 * t35137 * t2069;
    let t35145 = t31142 * t8884;
    let t35146 = 7.0_f64 / 72.0_f64 * t35145;
    let t35148 = t2019 * t8887 * t8889;
    let t35149 = 7.0_f64 / 72.0_f64 * t35148;
    let t35151 = t7436 * t142 * t5183;
    (t35139, t35146, t35149, t35151)
}
