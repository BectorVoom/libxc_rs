//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta895 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2853;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta895<F: Float>(t61178: F, t61180: F, t39860: F, t18263: F, t4305: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t49958: F, t49964: F, t49982: F, t190: F, t706: F, t76397: F, t40092: F, t40094: F, t14330: F, t18305: F, t4181: F, t61201: F, t157: F, t23121: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t76976, t76977, t76978, t76980, t76981) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2853::<F>(t61178, t61180, t39860, t18263, t4305, t39783, t39786, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t40084, t49958, t49964, t49982);
        let (t76986, t76987, t76988, t76991, t76992, t76995) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2854::<F>(t190, t706, t76397, t40092, t40094, t14330, t18305, t4181, t61201, t157, t23121, t606);
    (t76976, t76977, t76978, t76980, t76981, t76986, t76987, t76988, t76991, t76992, t76995)
}
