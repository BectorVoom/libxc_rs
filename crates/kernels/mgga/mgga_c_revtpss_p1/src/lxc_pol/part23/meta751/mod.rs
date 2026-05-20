//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta751 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta751<F: Float>(t11408: F, t1614: F, t2967: F, t4644: F, t11449: F, t11409: F, t1621: F, t2968: F, t300: F, t3012: F, t11507: F, t51973: F) -> (F, F, F, F, F, F, F, F) {
        let (t52812, t52820, t52825, t52837, t52840, t52877, t52894, t52946) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2540::<F>(t11408, t1614, t2967, t4644, t11449, t11409, t1621, t2968, t300, t3012, t11507, t51973);
    (t52812, t52820, t52825, t52837, t52840, t52877, t52894, t52946)
}
