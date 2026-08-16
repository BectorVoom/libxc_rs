//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta939 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3085;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta939<F: Float>(t24274: F, t698: F, t52011: F, t58027: F, t77513: F, t24271: F, t1134: F, t6449: F, t16851: F, t16854: F, t43888: F, t58153: F, t58165: F, t58543: F, t81242: F, t81245: F, t81489: F, t24317: F, t43821: F, t20356: F, t5079: F, t24312: F, t3390: F, t16857: F, t20337: F, t5071: F, t43946: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81491, t81494, t81496, t81499, t81501, t81506) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3085::<F>(t24274, t698, t52011, t58027, t77513, t24271, t1134, t6449, t16851, t16854, t43888, t58153, t58165, t58543, t81242, t81245, t81489);
        let (t81509, t81511, t81514, t81516, t81518, t81521) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3086::<F>(t1134, t24317, t43821, t20356, t5079, t24312, t3390, t16857, t6449, t20337, t5071, t43946);
    (t81491, t81494, t81496, t81499, t81501, t81506, t81509, t81511, t81514, t81516, t81518, t81521)
}
