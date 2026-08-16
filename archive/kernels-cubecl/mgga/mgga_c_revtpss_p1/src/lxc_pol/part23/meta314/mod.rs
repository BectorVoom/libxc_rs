//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1599;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1600;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1601;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta314<F: Float>(t12051: F, t471: F, t11239: F, t3596: F, t460: F, t3603: F, t13038: F) -> (F, F, F, F, F, F) {
        let (t13129, t13141) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1599::<F>(t12051, t471, t11239, t3596);
        let t13142 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1600::<F>(t13141, t460);
        let (t13143, t13147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1601::<F>(t12051, t3603, t11239, t13038);
        let t13148 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1602::<F>(t13147, t460);
    (t13129, t13141, t13142, t13143, t13147, t13148)
}
