//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk224;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk225;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk226;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk227;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk228;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk229;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta30<F: Float>(t25: F, t596: F, t88: F, t90: F, t29: F, t17: F, t2: F, t579: F, t66: F, t64: F, t45: F, t78: F, t57: F, t81: F, t116: F, t94: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t598, t602, t603, t604, t624) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk224::<F>(t25, t596, t88, t90, t29, t17, t2, t579, t66);
        let t625 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk225::<F>(t624, t64);
        let (t626, t631) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk226::<F>(t625, t45);
        let t633 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk227::<F>(t631, t78);
        let t635 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk228::<F>(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk229::<F>(t635, t81);
        let t651 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk230::<F>(t116, t94);
    (t598, t602, t603, t604, t624, t625, t626, t631, t633, t635, t637, t651)
}
