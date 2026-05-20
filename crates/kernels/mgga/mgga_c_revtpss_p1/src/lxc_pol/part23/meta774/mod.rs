//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta774 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta774<F: Float>(t56176: F, t56183: F, t56228: F, t2439: F, t5101: F, t1729: F, t9303: F, t5095: F, t5098: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58073, t58075, t58090, t58114, t58117, t58134, t58145, t58146, t58153, t58165, t58166, t58225) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2578::<F>(t56176, t56183, t56228, t2439, t5101, t1729, t9303, t5095, t5098);
    (t58073, t58075, t58090, t58114, t58117, t58134, t58145, t58146, t58153, t58165, t58166, t58225)
}
