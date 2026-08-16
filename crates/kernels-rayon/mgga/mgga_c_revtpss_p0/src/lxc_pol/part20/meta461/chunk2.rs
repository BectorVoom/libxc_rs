//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1755/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1755(t73: f64, t9940: f64, t13902: f64, t1392: f64, t1394: f64, t1395: f64, t225: f64, t3889: f64, t4045: f64, t4049: f64, t4050: f64, t4053: f64, t46298: f64, t46345: f64, t46590: f64, t46628: f64, t46966: f64, t46985: f64, t47004: f64, t47021: f64, t47080: f64, t47103: f64, t47129: f64, t47153: f64, t539: f64, t541: f64, t5650: f64, t9628: f64, t9737: f64, t9872: f64, t9881: f64, t9884: f64, t9887: f64) -> f64 {
    let t47171 = t73 * t9940;
    let t47187 = -(t46966 + t46985 + t47004 + t47021 + t47080 + t47103 + t47129 + t47153) * t225 * t541 + 12.0_f64 * t9872 * t1395 - 72.0_f64 * t4045 * t4050 + 18.0_f64 * t4045 * t4053 + 240.0_f64 * t1392 * t9881 - 144.0_f64 * t13902 * t9884 + 12.0_f64 * t1392 * t9887 - 360.0_f64 * t539 * t47171 * t46628 + 360.0_f64 * t5650 * t46590 * t3889 - 36.0_f64 * t539 * t4049 * t46298 - 48.0_f64 * t5650 * t9737 * t9628 + 3.0_f64 * t539 * t1394 * t46345;
    t47187
}
