//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta308 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1576;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1577;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1578;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1579;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1580;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta308<F: Float>(t1269: F, t1284: F, t1209: F, t1204: F, t3781: F, t5462: F, t5477: F, t3634: F, t828: F, t3624: F, t3746: F, t3618: F, t5330: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12722, t12723, t12744, t12751) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1576::<F>(t1269, t1284, t1209, t1204, t3781, t5462);
        let t12756 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1577::<F>(t1209, t5477);
        let t12772 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1578::<F>(t3634, t828);
        let t12784 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1579::<F>(t3624, t3746);
        let t12787 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1580::<F>(t3618, t828);
        let (t12808, t12809) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1581::<F>(t1209, t3781, t5330);
    (t12722, t12723, t12744, t12751, t12756, t12772, t12784, t12787, t12808, t12809)
}
