//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta676<F: Float>(t406: F, t43813: F, t1126: F, t12226: F, t3382: F, t3431: F, t408: F, t43816: F, t3434: F, t12247: F, t3800: F, t11239: F, t1204: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43946, t43995, t44012, t44017, t44039, t44040, t44091, t44093, t44101, t44126, t44173) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2413::<F>(t406, t43813, t1126, t12226, t3382, t3431, t408, t43816, t3434, t12247, t3800, t11239, t1204);
    (t43946, t43995, t44012, t44017, t44039, t44040, t44091, t44093, t44101, t44126, t44173)
}
