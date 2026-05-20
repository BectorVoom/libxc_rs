//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta232<F: Float>(t13126: F, t460: F, t12051: F, t471: F, t11239: F, t3596: F, t3603: F, t13038: F, t13045: F, t1275: F, t225: F, t1466: F, t2246: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13127, t13129, t13141, t13142, t13143, t13147, t13148, t13149, t13180, t13182, t13272) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk990::<F>(t13126, t460, t12051, t471, t11239, t3596, t3603, t13038, t13045, t1275, t225, t1466, t2246);
    (t13127, t13129, t13141, t13142, t13143, t13147, t13148, t13149, t13180, t13182, t13272)
}
