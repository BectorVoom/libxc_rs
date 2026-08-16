//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 992/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk992(t35113: f64, t1164: f64, t8853: f64, t31142: f64, t8884: f64, t2019: f64, t8887: f64, t8889: f64, t1992: f64, t30127: f64, t7842: f64, t8791: f64) -> (f64, f64, f64, f64, f64) {
    let t35114 = 0.94344276868812456204e-2_f64 * t35113;
    let t35137 = t1164 * t8853;
    let t35145 = t31142 * t8884;
    let t35146 = 7.0_f64 / 72.0_f64 * t35145;
    let t35148 = t2019 * t8887 * t8889;
    let t35149 = 7.0_f64 / 72.0_f64 * t35148;
    let t35176 = t30127 * t7842 * t1992 * t8791;
    (t35114, t35137, t35146, t35149, t35176)
}
