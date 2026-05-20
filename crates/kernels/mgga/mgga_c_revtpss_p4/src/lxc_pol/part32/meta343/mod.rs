//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta343<F: Float>(t14005: F, t9816: F, t2713: F, t3964: F, t5617: F, t5686: F, t9744: F, t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F) -> (F, F, F, F, F, F) {
        let (t14007, t14013, t14024, t14036, t14038, t14040) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1271::<F>(t14005, t9816, t2713, t3964, t5617, t5686, t9744, t221, t4019, t5659, t4018, t3989, t5629);
    (t14007, t14013, t14024, t14036, t14038, t14040)
}
