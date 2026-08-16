//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2260;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2261;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta605<F: Float>(t1733: F, t6470: F, t3384: F, t1732: F, t20644: F, t3433: F, t17092: F, t6439: F, t6438: F, t1150: F, t12256: F, t22688: F, t12305: F, t128: F, t12268: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24212, t24214, t24215, t24217, t24219, t24220, t24221, t24223, t24228) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2260::<F>(t1733, t6470, t3384, t1732, t20644, t3433, t17092, t6439, t6438, t1150, t12256, t22688);
        let (t24229, t24230) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2261::<F>(t12305, t24228, t128);
        let t24232 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2262::<F>(t12268, t22688);
    (t24212, t24214, t24215, t24217, t24219, t24220, t24221, t24223, t24228, t24229, t24230, t24232)
}
