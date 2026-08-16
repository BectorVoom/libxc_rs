//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk588;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk589;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk590;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk591;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta103(t3475: f64, t426: f64, t434: f64, t3356: f64, t1178: f64, t444: f64, t439: f64, t3413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3476, t3477) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk588(t3475, t426);
        let (t3478, t3479) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk589(t434);
        let (t3483, t3494, t3495) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk590(t3356, t1178, t444);
        let (t3496, t3503, t3510, t3519) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk591(t3495, t439, t3356, t3413, t1178);
        let t3520 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk592(t3519);
    (t3476, t3477, t3478, t3479, t3483, t3494, t3495, t3496, t3503, t3510, t3519, t3520)
}
