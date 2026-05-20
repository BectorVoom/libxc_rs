//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2069;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta596<F: Float>(t25875: F, t94762: F, t4004: F, t676: F, t25880: F, t25894: F, t25877: F, t94382: F, t94590: F, t25950: F, t26050: F, t25304: F, t25949: F, t25946: F, t25878: F, t94661: F, t7246: F, t9692: F, t26054: F, t9671: F, t1419: F, t7063: F, t25898: F, t25901: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94766, t94769, t94771, t94772, t94774, t94776) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2069::<F>(t25875, t94762, t4004, t676, t25880, t25894, t25877, t94382, t94590, t25950, t26050, t25304, t25949);
        let (t94777, t94779, t94784, t94799, t94801, t94802, t94803) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2070::<F>(t25946, t94776, t25878, t94661, t7246, t9692, t26054, t9671, t1419, t7063, t25898, t25901);
    (t94766, t94769, t94771, t94772, t94774, t94777, t94779, t94784, t94799, t94801, t94802, t94803)
}
