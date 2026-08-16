//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta554 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1650;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1651;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1652;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1653;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1654;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta554(t128: f64, t41339: f64, t88102: f64, t87126: f64, t905: f64, t904: f64, t41270: f64, t87145: f64, t11142: f64, t18903: f64, t5825: f64, t11144: f64, t2850: f64, t18908: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t88104 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1650(t128, t41339, t88102);
        let (t88106, t88108) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1651(t87126, t905, t128, t904);
        let (t88112, t88114) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1652(t41270, t87145, t11142, t128);
        let (t88116, t88118) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1653(t18903, t5825, t11142, t128);
        let (t88120, t88122) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1654(t11144, t87145, t128, t2850);
        let (t88124, t88126) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1655(t18908, t5825, t128, t2850);
    (t88104, t88106, t88108, t88112, t88114, t88116, t88118, t88120, t88122, t88124, t88126)
}
