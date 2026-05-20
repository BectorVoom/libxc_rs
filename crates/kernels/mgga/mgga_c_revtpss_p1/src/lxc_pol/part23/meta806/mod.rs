//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta806 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2638;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta806<F: Float>(t1558: F, t2482: F, t2801: F, t4526: F, t136: F, t2457: F, t39680: F, t6022: F, t10073: F, t18746: F, t18742: F, t10069: F, t231: F, t2782: F, t2783: F, t62868: F, t18729: F, t2470: F, t2798: F, t2723: F, t4503: F, t62760: F, t6016: F, t879: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t62881, t62907, t62909, t62920, t62922) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2638::<F>(t1558, t2482, t2801, t4526, t136, t2457, t39680, t6022, t10073, t18746, t18742, t10069);
        let (t62938, t62952, t62961, t62967) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2639::<F>(t231, t2782, t2783, t62868, t18729, t2470, t2798, t2723, t4503, t62760, t2482, t6016, t879);
    (t62881, t62907, t62909, t62920, t62922, t62938, t62952, t62961, t62967)
}
