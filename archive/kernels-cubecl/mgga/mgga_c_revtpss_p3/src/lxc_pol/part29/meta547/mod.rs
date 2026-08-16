//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta547<F: Float>(t26276: F, t9285: F, t25944: F, t136: F, t2457: F, t7531: F, t26069: F, t7515: F, t94879: F, t26230: F, t9685: F, t25878: F) -> (F, F, F, F, F, F, F) {
        let (t96255, t96257, t96259, t96260, t96262, t96264, t96265) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1884::<F>(t26276, t9285, t25944, t136, t2457, t7531, t26069, t7515, t94879, t26230, t9685, t25878);
    (t96255, t96257, t96259, t96260, t96262, t96264, t96265)
}
