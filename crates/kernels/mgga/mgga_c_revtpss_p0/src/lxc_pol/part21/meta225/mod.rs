//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1343;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1344;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1345;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1346;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1347;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta225<F: Float>(t3302: F, t3603: F, t1248: F, t5332: F, t1269: F, t1287: F, t1794: F, t487: F, t5284: F, t3781: F, t460: F, t471: F, t1811: F, t473: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5464, t5465, t5466) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1343::<F>(t3302, t3603, t1248, t5332);
        let (t5470, t5474, t5477) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1344::<F>(t1269, t1287, t1794, t487, t5284, t3781);
        let t5478 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1345::<F>(t460, t5477);
        let t5480 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1346::<F>(t1248, t3302, t471);
        let t5481 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1347::<F>(t5332, t5480);
        let t5486 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1348::<F>(t1811, t473);
    (t5464, t5465, t5466, t5470, t5474, t5477, t5478, t5480, t5481, t5486)
}
