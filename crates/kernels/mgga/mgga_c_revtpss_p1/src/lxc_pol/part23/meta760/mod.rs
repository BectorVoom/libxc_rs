//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2554;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta760<F: Float>(t15731: F, t3169: F, t12078: F, t53740: F, t12047: F, t16138: F, t372: F, t11671: F, t15925: F, t1063: F, t11986: F, t247: F, t4583: F, t1062: F, t43154: F, t16088: F, t342: F, t380: F, t16219: F, t3241: F, t11262: F, t4802: F, t4807: F, t11773: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54733, t54801, t54811, t54818, t54916, t54943) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2554::<F>(t15731, t3169, t12078, t53740, t12047, t16138, t372, t11671, t15925, t1063, t11986, t247, t4583);
        let (t54982, t55011, t55034, t55062, t55065, t55141) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2555::<F>(t1062, t43154, t16088, t342, t380, t16219, t3241, t1063, t11262, t4802, t4807, t11773, t15925);
    (t54733, t54801, t54811, t54818, t54916, t54943, t54982, t55011, t55034, t55062, t55065, t55141)
}
