//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1877;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta410<F: Float>(t13126: F, t460: F, t13043: F, t487: F, t12051: F, t471: F, t3727: F, t473: F, t1214: F, t11239: F, t3596: F, t3603: F, t13038: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13127, t13128, t13129, t13130, t13133, t13134, t13141) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1877::<F>(t13126, t460, t13043, t487, t12051, t471, t3727, t473, t1214, t11239, t3596);
        let (t13142, t13143, t13144, t13147) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1878::<F>(t13141, t460, t12051, t3603, t13128, t11239, t13038);
    (t13127, t13128, t13129, t13130, t13133, t13134, t13141, t13142, t13143, t13144, t13147)
}
