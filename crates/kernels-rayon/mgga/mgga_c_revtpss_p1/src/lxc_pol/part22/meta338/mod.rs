//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1800;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1801;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1802;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta338(t159: f64, t3181: f64, t2851: f64, t631: f64, t45: f64, t1071: f64, t3057: f64, t3259: f64, t994: f64, t342: f64, t992: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11142 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1800(t159, t3181);
        let t11144 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1801(t2851, t631);
        let (t11149, t11150) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1802(t2851, t45);
        let (t11187, t11190, t11195, t11198, t11199, t11200) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1803(t1071, t3057, t3259, t994, t342, t992, t338);
    (t11142, t11144, t11149, t11150, t11187, t11190, t11195, t11198, t11199, t11200)
}
