//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk706;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk707;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk708;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk709;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk710;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk711;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk712;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta134<F: Float>(t1260: F, t3670: F, t1802: F, t369: F, t475: F, t467: F, t1811: F, t460: F, t1284: F, t1770: F, t354: F, t471: F, t3766: F, t487: F, t3302: F, t3603: F, t3781: F, t473: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5384 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk706::<F>(t1260, t3670);
        let t5390 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk707::<F>(t1802, t369, t475);
        let t5391 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk708::<F>(t467, t5390);
        let t5417 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk709::<F>(t1811, t460);
        let t5436 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk710::<F>(t1284, t1770);
        let (t5457, t5462) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk711::<F>(t354, t471, t3766, t487);
        let (t5463, t5464, t5477) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk712::<F>(t460, t5462, t3302, t3603, t3781, t487);
        let (t5478, t5486) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk713::<F>(t460, t5477, t1811, t473);
    (t5384, t5390, t5391, t5417, t5436, t5457, t5462, t5463, t5464, t5477, t5478, t5486)
}
