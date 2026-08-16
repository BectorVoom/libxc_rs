//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta965 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3262;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta965(t73352: f64, t177: f64, t22789: f64, t762: f64, t48227: f64, t46973: f64, t48243: f64, t46977: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t48224: f64, t48226: f64, t48234: f64, t48236: f64, t48241: f64, t48244: f64, t48248: f64, t73374: f64, t46989: f64, t46993: f64, t47005: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39773: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t47003: f64, t48256: f64, t48259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85893, t85896, t85897, t85898, t85899, t85900, t85901) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3262(t73352, t177, t22789, t762, t48227, t46973, t48243, t46977, t39483, t39520, t39528, t39531, t48224, t48226, t48234, t48236, t48241, t48244, t48248);
        let (t85903, t85904, t85905, t85906, t85907) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3263(t73374, t46989, t46993, t47005, t39747, t39750, t39756, t39760, t39773, t46988, t46992, t46996, t46998, t47003, t48256, t48259);
    (t85893, t85896, t85897, t85898, t85899, t85900, t85901, t85903, t85904, t85905, t85906, t85907)
}
