//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1753;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1754;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta461(t3869: f64, t39532: f64, t123: f64, t2630: f64, t3850: f64, t9575: f64, t9860: f64, t39538: f64, t39427: f64, t39535: f64, t187: f64, t47055: f64, t2496: f64, t9551: f64, t4038: f64, t9372: f64, t1317: f64, t9428: f64, t3853: f64, t3857: f64, t40076: f64, t40079: f64, t73: f64, t9940: f64, t13902: f64, t1392: f64, t1394: f64, t1395: f64, t225: f64, t3889: f64, t4045: f64, t4049: f64, t4050: f64, t4053: f64, t46298: f64, t46345: f64, t46590: f64, t46628: f64, t46966: f64, t46985: f64, t47004: f64, t47021: f64, t47080: f64, t47103: f64, t47129: f64, t539: f64, t541: f64, t5650: f64, t9628: f64, t9737: f64, t9872: f64, t9881: f64, t9884: f64, t9887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47131, t47134, t47136, t47138, t47140, t47142, t47144) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1753(t3869, t39532, t123, t2630, t3850, t9575, t9860, t39538, t39427, t39535, t187, t47055);
        let (t47146, t47148, t47150, t47152, t47153) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1754(t2496, t9551, t4038, t9372, t1317, t9428, t3853, t3857, t40076, t40079, t47131, t47134, t47136, t47138, t47140, t47142, t47144);
        let t47187 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1755(t73, t9940, t13902, t1392, t1394, t1395, t225, t3889, t4045, t4049, t4050, t4053, t46298, t46345, t46590, t46628, t46966, t46985, t47004, t47021, t47080, t47103, t47129, t47153, t539, t541, t5650, t9628, t9737, t9872, t9881, t9884, t9887);
    (t47131, t47134, t47136, t47138, t47140, t47142, t47144, t47146, t47148, t47150, t47152, t47187)
}
