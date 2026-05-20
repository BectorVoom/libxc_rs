//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta139 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk775;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk776;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk777;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk778;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta139<F: Float>(t3737: F, t3738: F, t1204: F, t1284: F, t1280: F, t3568: F, t487: F, t1209: F, t1287: F, t3721: F, t1269: F, t473: F, t1214: F, t3584: F, t3140: F, t3596: F, t460: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3739, t3746) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk775::<F>(t3737, t3738, t1204, t1284);
        let (t3751, t3754) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk776::<F>(t1280, t3568, t1284, t487);
        let (t3755, t3756, t3759) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk777::<F>(t1209, t3754, t1287, t3721, t1269, t473);
        let (t3760, t3763, t3766) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk778::<F>(t1214, t3759, t1280, t3584, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk779::<F>(t3766, t460);
    (t3739, t3746, t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766, t3767)
}
