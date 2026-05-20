//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta631<F: Float>(t3252: F, t65: F, t1100: F, t1699: F, t1448: F, t1907: F, t4292: F, t93: F, t1224: F, t3698: F, t1298: F, t1832: F) -> (F, F, F, F, F, F, F) {
        let (t27531, t27717, t28198, t28219, t29048, t29054, t29322) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2325::<F>(t3252, t65, t1100, t1699, t1448, t1907, t4292, t93, t1224, t3698, t1298, t1832);
    (t27531, t27717, t28198, t28219, t29048, t29054, t29322)
}
