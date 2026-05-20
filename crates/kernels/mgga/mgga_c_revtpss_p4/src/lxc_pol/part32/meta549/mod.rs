//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta549<F: Float>(t94522: F, t94525: F, t94568: F, t94570: F, t7284: F, t96282: F, t26277: F, t94913: F, t25944: F, t96259: F, t1385: F, t7506: F) -> (F, F, F, F, F, F, F, F) {
        let (t96341, t96342, t96358, t96359, t96374, t96380, t96382, t96392) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1864::<F>(t94522, t94525, t94568, t94570, t7284, t96282, t26277, t94913, t25944, t96259, t1385, t7506);
    (t96341, t96342, t96358, t96359, t96374, t96380, t96382, t96392)
}
