//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk612;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk613;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk614;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk615;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk616;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk617;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk618;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta107(t3362: f64, t3698: f64, t1234: f64, t1260: f64, t1209: f64, t1284: f64, t3624: f64, t482: f64, t66: f64, t828: f64, t1275: f64, t493: f64, t225: f64, t487: f64, t3140: f64, t3596: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3699, t3711) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk612(t3362, t3698, t1234, t1260);
        let t3717 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk613(t1209, t1284);
        let t3718 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk614(t3624, t3717);
        let (t3719, t3720) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk615(t482, t66, t828);
        let (t3736, t3737) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk616(t1275, t493, t225);
        let t3754 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk617(t1284, t487);
        let (t3755, t3766) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk618(t1209, t3754, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk619(t3766, t460);
    (t3699, t3711, t3717, t3718, t3719, t3720, t3736, t3737, t3754, t3755, t3766, t3767)
}
