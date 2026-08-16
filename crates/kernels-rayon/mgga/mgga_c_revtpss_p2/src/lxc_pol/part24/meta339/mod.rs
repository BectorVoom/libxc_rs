//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1185;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1186;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta339(t23485: f64, t904: f64, t128: f64, t23474: f64, t23481: f64, t2908: f64, t141: f64, t930: f64, t4573: f64, t5825: f64, t2850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23486, t23487) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1185(t23485, t904, t128);
        let (t23489, t23490) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1186(t23474, t904, t128);
        let (t23492, t23493, t23495, t23496, t23499, t23500, t23501) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1187(t23481, t2908, t141, t23485, t930, t4573, t5825, t2850, t128);
    (t23486, t23487, t23489, t23490, t23492, t23493, t23495, t23496, t23499, t23500, t23501)
}
