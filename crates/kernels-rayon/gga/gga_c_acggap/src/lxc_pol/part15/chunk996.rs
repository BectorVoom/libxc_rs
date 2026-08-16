//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 996/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk996(t30924: f64, t30928: f64, t1164: f64, t8853: f64, t31142: f64, t8884: f64, t2019: f64, t8887: f64, t8889: f64, t30978: f64, t30982: f64, t30985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35123 = 0.75475421495049964964e-2_f64 * t30924;
    let t35125 = 0.75475421495049964964e-2_f64 * t30928;
    let t35137 = t1164 * t8853;
    let t35145 = t31142 * t8884;
    let t35148 = t2019 * t8887 * t8889;
    let t35160 = 0.16006300097412701803e-1_f64 * t30978;
    let t35162 = 0.16006300097412701803e-1_f64 * t30982;
    let t35163 = 0.21437009059034868486e-2_f64 * t30985;
    (t35123, t35125, t35137, t35145, t35148, t35160, t35162, t35163)
}
