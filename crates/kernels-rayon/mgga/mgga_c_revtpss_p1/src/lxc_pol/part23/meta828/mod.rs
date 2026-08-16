//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta828 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2685;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta828(t1025: f64, t371: f64, t6276: f64, t676: f64, t15749: f64, t4858: f64, t11789: f64, t20016: f64, t3205: f64, t6337: f64, t15666: f64, t1053: f64, t19463: f64, t11921: f64, t19414: f64, t247: f64, t4837: f64, t11710: f64, t20078: f64, t3091: f64, t11922: f64, t11927: f64, t19621: f64, t11774: f64, t4787: f64, t53391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67186, t67195, t67199, t67206, t67213, t67215) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2685(t1025, t371, t6276, t676, t15749, t4858, t11789, t20016, t3205, t6337, t15666, t1053, t19463);
        let (t67237, t67249, t67253, t67264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2686(t11921, t19414, t247, t4837, t11710, t20078, t3091, t11922, t11927, t19621, t11774, t4787, t53391);
    (t67186, t67195, t67199, t67206, t67213, t67215, t67237, t67249, t67253, t67264)
}
