//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1657;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1658;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1659;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta555<F: Float>(t2852: F, t87107: F, t128: F, t2850: F, t22671: F, t4573: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t291: F, t88100: F, t141: F, t41294: F, t88102: F, t88106: F, t930: F, t11341: F, t88112: F, t2908: F, t88120: F, t41246: F, t77499: F, t77663: F, t77667: F, t88089: F, t88097: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t88128, t88130) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1656::<F>(t2852, t87107, t128, t2850);
        let (t88132, t88134) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1657::<F>(t22671, t4573, t128, t2850);
        let t88137 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1658::<F>(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88140, t88144, t88147, t88150, t88161) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1659::<F>(t291, t88100, t88137, t141, t41294, t88102, t88106, t930, t11341, t88112, t2908, t88120);
        let (t88164, t88166) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1660::<F>(t141, t2908, t88128, t41246, t77499, t77505, t77507, t77509, t77663, t77667, t88089, t88097, t88144, t88147, t88150, t88161);
    (t88128, t88130, t88132, t88134, t88140, t88144, t88147, t88150, t88161, t88164, t88166)
}
