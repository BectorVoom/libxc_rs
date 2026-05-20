//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2543;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta754<F: Float>(t11821: F, t65: F, t11144: F, t11970: F, t1660: F, t27527: F, t2852: F, t11150: F, t27531: F, t127: F, t4823: F, t15690: F, t247: F, t42792: F, t4757: F, t4837: F, t3091: F, t43240: F, t4782: F, t41296: F, t42471: F, t3155: F, t999: F, t1011: F, t4886: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53322, t53326, t53328, t53332, t53391, t53405) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2543::<F>(t11821, t65, t11144, t11970, t1660, t27527, t2852, t11150, t27531, t127, t4823, t15690);
        let (t53432, t53437, t53473, t53511, t53542) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2544::<F>(t247, t42792, t4757, t4837, t3091, t43240, t4782, t41296, t42471, t3155, t999, t1011, t4886, t697);
    (t53322, t53326, t53328, t53332, t53391, t53405, t53432, t53437, t53473, t53511, t53542)
}
