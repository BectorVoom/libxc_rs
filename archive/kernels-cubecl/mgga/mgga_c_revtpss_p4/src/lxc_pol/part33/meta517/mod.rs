//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1854;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta517<F: Float>(t27384: F, t27799: F, t1113: F, t1583: F, t33: F, t4537: F, t1711: F, t775: F, t890: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t27158: F, t27364: F, t27368: F, t27382: F, t27407: F, t27764: F, t27770: F, t27773: F, t27777: F, t27793: F, t7087: F, t7091: F, t7200: F, t7207: F, t7783: F, t7862: F, t7869: F, t196: F, t197: F, t5528: F, t2035: F, t7313: F, t7898: F, t1032: F, t1892: F, t1955: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27800, t27802, t27806, t27810, t27817, t27821) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1854::<F>(t27384, t27799, t1113, t1583, t33, t4537, t1711, t775, t890, t1940, t1963, t2403, t25206, t25440, t27158, t27364, t27368, t27382, t27407, t27764, t27770, t27773, t27777, t27793, t7087, t7091, t7200, t7207, t7783, t7862, t7869);
        let (t27833, t27834, t27835, t27836, t27837) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1855::<F>(t196, t197, t5528, t2035, t7313, t7898, t1032, t1892, t1955);
    (t27800, t27802, t27806, t27810, t27817, t27821, t27833, t27834, t27835, t27836, t27837)
}
