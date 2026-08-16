//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta825 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2679;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta825<F: Float>(t1668: F, t372: F, t4823: F, t1043: F, t11249: F, t11866: F, t19976: F, t19907: F, t3241: F, t1011: F, t6288: F, t697: F, t11710: F, t19872: F, t3091: F, t19968: F, t3111: F, t15850: F, t4817: F, t11921: F, t19399: F, t247: F, t4837: F, t15752: F, t19741: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t66689, t66702, t66712, t66714, t66721) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2679::<F>(t1668, t372, t4823, t1043, t11249, t11866, t19976, t19907, t3241, t1011, t6288, t697);
        let (t66731, t66739, t66747, t66752, t66758) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2680::<F>(t11710, t19872, t3091, t19968, t3111, t15850, t4817, t11921, t19399, t247, t4837, t15752, t19741);
    (t66689, t66702, t66712, t66714, t66721, t66731, t66739, t66747, t66752, t66758)
}
