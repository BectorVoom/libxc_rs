//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1770;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta495<F: Float>(t2055: F, t5517: F, t72: F, t8094: F, t686: F, t25878: F, t25895: F, t1882: F, t543: F, t7506: F, t7301: F, t27884: F, t7515: F, t25921: F, t26232: F, t26235: F, t26238: F, t26251: F, t26253: F, t26263: F, t26266: F, t26268: F, t26272: F, t7295: F, t8100: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28760, t28779, t28780) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1770::<F>(t2055, t5517, t72, t8094, t686);
        let (t28781, t28783, t28791, t28792, t28796, t28799) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1771::<F>(t25878, t28780, t25895, t1882, t543, t7506, t7301, t27884, t7515, t25921, t26232, t26235, t26238, t26251, t26253, t26263, t26266, t26268, t26272, t7295, t8100);
    (t28760, t28779, t28780, t28781, t28783, t28791, t28792, t28796, t28799)
}
