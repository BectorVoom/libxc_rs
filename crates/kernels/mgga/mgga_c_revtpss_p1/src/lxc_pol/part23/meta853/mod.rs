//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta853 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2739;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2740;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta853<F: Float>(t17609: F, t5265: F, t17544: F, t5274: F, t1222: F, t17471: F, t20298: F, t20302: F, t1260: F, t57465: F, t21334: F, t17763: F, t5378: F, t12855: F, t12916: F, t20977: F, t20913: F, t3172: F, t3711: F, t21107: F, t3704: F, t17628: F, t5373: F, t20851: F, t3678: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71550, t71552, t71571, t71582, t71585, t71590, t71598) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2739::<F>(t17609, t5265, t17544, t5274, t1222, t17471, t20298, t20302, t1260, t57465, t21334, t17763, t5378);
        let (t71630, t71687, t71710, t71718, t71738) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2740::<F>(t12855, t12916, t20977, t20913, t3172, t3711, t21107, t3704, t17628, t5373, t20851, t3678);
    (t71550, t71552, t71571, t71582, t71585, t71590, t71598, t71630, t71687, t71710, t71718, t71738)
}
