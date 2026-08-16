//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta43 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk296;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk297;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk298;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk299;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta43(t315: f64, t964: f64, t902: f64, t928: f64, t323: f64, t300: f64, t340: f64, t338: f64, t378: f64, t225: f64, t385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t965, t967, t970, t973) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk296(t315, t964, t902, t928, t323);
        let t981 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk297(t300, t315);
        let (t986, t992, t993, t994) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk298(t902, t340, t338);
        let t995 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk299(t378, t994);
        let t996 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk300(t225, t385);
    (t965, t967, t970, t973, t981, t986, t992, t993, t994, t995, t996)
}
