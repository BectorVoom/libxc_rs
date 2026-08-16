//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk559;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk560;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk561;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk562;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk563;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta97(t3140: f64, t342: f64, t1034: f64, t358: f64, t360: f64, t368: f64, t335: f64, t365: f64, t73: f64, t357: f64, t1036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3141, t3143, t3144, t3145) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk559(t3140, t342, t1034, t358, t360, t368);
        let t3147 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk560(t3145, t335);
        let (t3148, t3149, t3150, t3153) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk561(t3147, t365, t3144, t3141, t73);
        let t3154 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk562(t357);
        let t3155 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk563(t3153, t3154);
        let (t3160, t3161, t3162) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk564(t1036, t3148, t3141, t3153, t357);
    (t3143, t3144, t3145, t3147, t3149, t3150, t3153, t3154, t3155, t3160, t3161, t3162)
}
