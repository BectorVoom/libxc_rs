//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2131;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta458<F: Float>(t2926: F, t4631: F, t934: F, t2924: F, t2918: F, t4635: F, t11387: F, t1609: F, t2875: F, t11385: F, t4644: F, t945: F, t11456: F, t15350: F, t15373: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t1634: F, t2982: F, t3015: F, t311: F, t4708: F, t955: F) -> (F, F, F, F, F, F, F, F) {
        let (t15390, t15392, t15393, t15395, t15397, t15399, t15400) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2131::<F>(t2926, t4631, t934, t2924, t2918, t4635, t11387, t1609, t2875, t11385, t4644, t945);
        let t15403 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2132::<F>(t11456, t15350, t15373, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15400, t1634, t2982, t3015, t311, t4708, t955);
    (t15390, t15392, t15393, t15395, t15397, t15399, t15400, t15403)
}
