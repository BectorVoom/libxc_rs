//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1552;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1553;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1554;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta300<F: Float>(t1052: F, t3147: F, t1036: F, t3141: F, t3144: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F, t11249: F, t357: F, t3143: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11998, t11999, t12012, t12013, t12046) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1552::<F>(t1052, t3147, t1036, t3141, t3144, t1035, t11239);
        let (t12047, t12050) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1553::<F>(t12046, t342, t3145, t334);
        let t12051 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1554::<F>(t11249, t12050);
        let (t12052, t12077) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1555::<F>(t12051, t357, t11239, t3143);
    (t11998, t11999, t12012, t12013, t12046, t12047, t12050, t12051, t12052, t12077)
}
