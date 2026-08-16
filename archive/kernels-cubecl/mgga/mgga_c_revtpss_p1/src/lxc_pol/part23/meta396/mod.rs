//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta396<F: Float>(t17471: F, t5047: F, t1222: F, t1012: F, t13026: F, t1263: F, t5245: F, t1234: F, t5390: F) -> (F, F, F, F, F) {
        let (t17472, t17474, t17475, t17500, t17505) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1755::<F>(t17471, t5047, t1222, t1012, t13026, t1263, t5245, t1234, t5390);
    (t17472, t17474, t17475, t17500, t17505)
}
