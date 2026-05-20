//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta529<F: Float>(t25110: F, t26179: F, t26169: F, t6963: F, t45963: F, t7342: F, t10301: F, t26178: F, t6960: F, t25114: F, t25102: F, t7349: F) -> (F, F, F, F, F, F, F) {
        let (t95268, t95270, t95276, t95283, t95284, t95286, t95288) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1858::<F>(t25110, t26179, t26169, t6963, t45963, t7342, t10301, t26178, t6960, t25114, t25102, t7349);
    (t95268, t95270, t95276, t95283, t95284, t95286, t95288)
}
