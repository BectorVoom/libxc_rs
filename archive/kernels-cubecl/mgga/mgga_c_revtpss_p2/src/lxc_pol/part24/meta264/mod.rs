//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1035;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta264<F: Float>(t17376: F, t3599: F, t1285: F, t17395: F, t1781: F, t697: F, t1222: F, t3367: F, t471: F, t372: F, t5296: F, t17350: F, t3767: F, t5277: F, t3362: F, t12865: F, t5302: F, t15904: F, t3623: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17572, t17605, t17628, t17629, t17643, t17649, t17654) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1035::<F>(t17376, t3599, t1285, t17395, t1781, t697, t1222, t3367, t471, t372, t5296, t17350, t3767);
        let (t17661, t17687, t17693, t17694, t17708) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1036::<F>(t372, t5277, t3362, t471, t1285, t12865, t5302, t15904, t3623);
    (t17572, t17605, t17628, t17629, t17643, t17649, t17654, t17661, t17687, t17693, t17694, t17708)
}
