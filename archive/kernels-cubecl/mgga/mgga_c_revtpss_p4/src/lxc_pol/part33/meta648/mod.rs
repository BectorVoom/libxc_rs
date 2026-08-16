//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2097;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta648<F: Float>(t17435: F, t7613: F, t17303: F, t29062: F, t3678: F, t17209: F, t26880: F, t29019: F, t3707: F, t26873: F, t5265: F, t15687: F, t26865: F, t3767: F, t3782: F, t1224: F, t139: F, t29047: F, t5052: F, t3698: F, t5047: F, t26866: F, t5436: F, t17225: F, t7624: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t104817, t104825, t104828, t104833, t104834, t104844, t104852) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2097::<F>(t17435, t7613, t17303, t29062, t3678, t17209, t26880, t29019, t3707, t26873, t5265, t15687, t26865);
        let (t104853, t104856, t104863, t104872, t104888, t104894) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2098::<F>(t104852, t3767, t3782, t1224, t139, t29047, t5052, t3698, t5047, t26866, t5436, t17225, t7624);
    (t104817, t104825, t104828, t104833, t104834, t104844, t104853, t104856, t104863, t104872, t104888, t104894)
}
