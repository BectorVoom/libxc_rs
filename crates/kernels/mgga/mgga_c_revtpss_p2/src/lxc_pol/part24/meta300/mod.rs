//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta300<F: Float>(t1222: F, t21169: F, t1234: F, t6594: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F, t1261: F, t3670: F, t5390: F) -> (F, F, F, F, F, F, F) {
        let (t21170, t21177, t21188, t21189, t21192, t21193, t21203) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1085::<F>(t1222, t21169, t1234, t6594, t3172, t6630, t3600, t247, t3634, t6425, t1261, t3670, t5390);
    (t21170, t21177, t21188, t21189, t21192, t21193, t21203)
}
