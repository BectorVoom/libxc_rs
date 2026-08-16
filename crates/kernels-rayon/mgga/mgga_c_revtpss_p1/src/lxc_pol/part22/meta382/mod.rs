//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1947;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta382(t13581: f64, t187: f64, t1857: f64, t3857: f64, t5591: f64, t566: f64, t9375: f64, t177: f64, t5566: f64, t762: f64, t1450: f64, t5778: f64, t3889: f64, t5537: f64, t1353: f64, t1868: f64, t3829: f64, t4139: f64, t5532: f64, t5536: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t9547: f64, t9599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13583, t13584, t13585, t13586, t13593, t13597, t13599, t13600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1947(t13581, t187, t1857, t3857, t5591, t566, t9375, t177, t5566, t762, t1450, t5778);
        let t13610 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1948(t3889, t5537, t1353, t13583, t13585, t13586, t13593, t13599, t13600, t1868, t3829, t4139, t5532, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391, t9547, t9599);
    (t13583, t13584, t13585, t13593, t13597, t13599, t13600, t13610)
}
