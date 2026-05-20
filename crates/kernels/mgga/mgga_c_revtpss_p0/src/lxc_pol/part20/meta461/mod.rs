//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1753;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1754;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta461<F: Float>(t3869: F, t39532: F, t123: F, t2630: F, t3850: F, t9575: F, t9860: F, t39538: F, t39427: F, t39535: F, t187: F, t47055: F, t2496: F, t9551: F, t4038: F, t9372: F, t1317: F, t9428: F, t3853: F, t3857: F, t40076: F, t40079: F, t73: F, t9940: F, t13902: F, t1392: F, t1394: F, t1395: F, t225: F, t3889: F, t4045: F, t4049: F, t4050: F, t4053: F, t46298: F, t46345: F, t46590: F, t46628: F, t46966: F, t46985: F, t47004: F, t47021: F, t47080: F, t47103: F, t47129: F, t539: F, t541: F, t5650: F, t9628: F, t9737: F, t9872: F, t9881: F, t9884: F, t9887: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47131, t47134, t47136, t47138, t47140, t47142, t47144) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1753::<F>(t3869, t39532, t123, t2630, t3850, t9575, t9860, t39538, t39427, t39535, t187, t47055);
        let (t47146, t47148, t47150, t47152, t47153) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1754::<F>(t2496, t9551, t4038, t9372, t1317, t9428, t3853, t3857, t40076, t40079, t47131, t47134, t47136, t47138, t47140, t47142, t47144);
        let t47187 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1755::<F>(t73, t9940, t13902, t1392, t1394, t1395, t225, t3889, t4045, t4049, t4050, t4053, t46298, t46345, t46590, t46628, t46966, t46985, t47004, t47021, t47080, t47103, t47129, t47153, t539, t541, t5650, t9628, t9737, t9872, t9881, t9884, t9887);
    (t47131, t47134, t47136, t47138, t47140, t47142, t47144, t47146, t47148, t47150, t47152, t47187)
}
