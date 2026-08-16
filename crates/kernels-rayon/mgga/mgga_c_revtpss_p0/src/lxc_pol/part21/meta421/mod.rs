//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1910;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta421(t3889: f64, t5537: f64, t1353: f64, t13583: f64, t13585: f64, t13586: f64, t13593: f64, t13599: f64, t13600: f64, t1868: f64, t3829: f64, t4139: f64, t5532: f64, t5536: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t9547: f64, t9599: f64, t2516: f64, t5571: f64, t5566: f64, t72: f64, t757: f64, t1320: f64, t5567: f64, t5569: f64, t9395: f64, t9398: f64, t1448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13607, t13610) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1910(t3889, t5537, t1353, t13583, t13585, t13586, t13593, t13599, t13600, t1868, t3829, t4139, t5532, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391, t9547, t9599);
        let (t13612, t13613, t13615, t13620, t13622, t13623, t13624, t13625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1911(t2516, t5571, t5566, t72, t757, t1320, t5567, t5569, t9395, t9398, t1353, t1448);
    (t13607, t13610, t13612, t13613, t13615, t13620, t13622, t13623, t13624, t13625)
}
