//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1033;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta263<F: Float>(t140: F, t3698: F, t1012: F, t13026: F, t1234: F, t5390: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t12268: F, t3617: F, t1260: F, t5326: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17471, t17475, t17505, t17524, t17525, t17528, t17529, t17550) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1033::<F>(t140, t3698, t1012, t13026, t1234, t5390, t1802, t3147, t3597, t3594, t1244, t12268, t3617);
        let t17569 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1034::<F>(t1260, t5326);
    (t17471, t17475, t17505, t17524, t17525, t17528, t17529, t17550, t17569)
}
