//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk733;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta143<F: Float>(t38: F, t5854: F, t2299: F, t5819: F, t5825: F, t633: F, t2306: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t5820: F, t5827: F, t5830: F, t71: F, t85: F, t5: F, t1497: F, t2247: F, t4173: F, t5812: F, t5816: F, t603: F, t91: F, t117: F, t1518: F) -> (F, F, F, F, F, F) {
        let (t5855, t5869, t5872) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk733::<F>(t38, t5854, t2299, t5819, t5825, t633, t2306, t637, t77, t1471, t1487, t1494, t5820, t5827, t5830, t71, t85);
        let (t5876, t5877, t5883) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk734::<F>(t5, t1497, t2247, t4173, t5812, t5816, t5872, t603, t91, t117, t1518);
    (t5855, t5869, t5872, t5876, t5877, t5883)
}
