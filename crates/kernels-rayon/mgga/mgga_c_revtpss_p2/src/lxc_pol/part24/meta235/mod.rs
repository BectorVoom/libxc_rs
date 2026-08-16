//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk994;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta235(t1408: f64, t240: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t828: f64, t9954: f64, t3935: f64, t1882: f64, t4003: f64, t1873: f64, t9741: f64, t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13767, t13779, t13781, t13783, t13789, t13790) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk994(t1408, t240, t5610, t9775, t1889, t9779, t828, t9954, t3935, t1882, t4003);
        let (t13798, t13800, t13801, t13804, t13845, t13846) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk995(t1873, t9741, t5651, t808, t9736, t241, t820, t9991, t2482, t4000, t814, t136, t550);
    (t13767, t13779, t13781, t13783, t13789, t13790, t13798, t13800, t13801, t13804, t13845, t13846)
}
