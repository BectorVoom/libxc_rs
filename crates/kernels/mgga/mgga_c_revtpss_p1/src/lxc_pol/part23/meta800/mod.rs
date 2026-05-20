//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta800 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2626;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta800<F: Float>(t18615: F, t231: F, t243: F, t2661: F, t2662: F, t14923: F, t18478: F, t10811: F, t18334: F, t18629: F, t10777: F, t10779: F, t14671: F, t18637: F, t50412: F, t6035: F, t4321: F, t4534: F, t689: F, t10995: F, t18312: F, t686: F, t72: F, t18804: F, t2470: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t62458, t62460, t62475, t62494, t62498) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2626::<F>(t18615, t231, t243, t2661, t2662, t14923, t18478, t10811, t18334, t18629, t10777, t10779, t14671, t18637);
        let (t62502, t62516, t62523, t62528) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2627::<F>(t10777, t10779, t50412, t6035, t4321, t4534, t689, t10995, t18312, t686, t72, t18804, t2470);
    (t62458, t62460, t62475, t62494, t62498, t62502, t62516, t62523, t62528)
}
