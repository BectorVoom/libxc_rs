//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta501<F: Float>(t1450: F, t6816: F, t6836: F, t196: F, t197: F, t6773: F, t5920: F, t94: F, t21663: F, t38: F, t5868: F, t76: F) -> (F, F, F, F, F, F) {
        let (t29494, t29498, t29506, t29508, t29513, t29532) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1787::<F>(t1450, t6816, t6836, t196, t197, t6773, t5920, t94, t21663, t38, t5868, t76);
    (t29494, t29498, t29506, t29508, t29513, t29532)
}
