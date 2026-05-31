//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1755/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1755<F: Float>(t73: F, t9940: F, t13902: F, t1392: F, t1394: F, t1395: F, t225: F, t3889: F, t4045: F, t4049: F, t4050: F, t4053: F, t46298: F, t46345: F, t46590: F, t46628: F, t46966: F, t46985: F, t47004: F, t47021: F, t47080: F, t47103: F, t47129: F, t47153: F, t539: F, t541: F, t5650: F, t9628: F, t9737: F, t9872: F, t9881: F, t9884: F, t9887: F) -> F {
    let t47171 = t73 * t9940;
    let t47187 = -(t46966 + t46985 + t47004 + t47021 + t47080 + t47103 + t47129 + t47153) * t225 * t541 + F::cast_from(12.0_f64) * t9872 * t1395 - F::cast_from(72.0_f64) * t4045 * t4050 + F::cast_from(18.0_f64) * t4045 * t4053 + F::cast_from(240.0_f64) * t1392 * t9881 - F::cast_from(144.0_f64) * t13902 * t9884 + F::cast_from(12.0_f64) * t1392 * t9887 - F::cast_from(360.0_f64) * t539 * t47171 * t46628 + F::cast_from(360.0_f64) * t5650 * t46590 * t3889 - F::cast_from(36.0_f64) * t539 * t4049 * t46298 - F::cast_from(48.0_f64) * t5650 * t9737 * t9628 + F::cast_from(3.0_f64) * t539 * t1394 * t46345;
    t47187
}
