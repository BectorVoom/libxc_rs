//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta828 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2685;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta828<F: Float>(t1025: F, t371: F, t6276: F, t676: F, t15749: F, t4858: F, t11789: F, t20016: F, t3205: F, t6337: F, t15666: F, t1053: F, t19463: F, t11921: F, t19414: F, t247: F, t4837: F, t11710: F, t20078: F, t3091: F, t11922: F, t11927: F, t19621: F, t11774: F, t4787: F, t53391: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t67186, t67195, t67199, t67206, t67213, t67215) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2685::<F>(t1025, t371, t6276, t676, t15749, t4858, t11789, t20016, t3205, t6337, t15666, t1053, t19463);
        let (t67237, t67249, t67253, t67264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2686::<F>(t11921, t19414, t247, t4837, t11710, t20078, t3091, t11922, t11927, t19621, t11774, t4787, t53391);
    (t67186, t67195, t67199, t67206, t67213, t67215, t67237, t67249, t67253, t67264)
}
