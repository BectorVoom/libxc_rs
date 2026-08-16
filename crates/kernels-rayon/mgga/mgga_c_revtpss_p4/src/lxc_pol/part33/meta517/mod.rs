//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1854;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta517(t27384: f64, t27799: f64, t1113: f64, t1583: f64, t33: f64, t4537: f64, t1711: f64, t775: f64, t890: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t27158: f64, t27364: f64, t27368: f64, t27382: f64, t27407: f64, t27764: f64, t27770: f64, t27773: f64, t27777: f64, t27793: f64, t7087: f64, t7091: f64, t7200: f64, t7207: f64, t7783: f64, t7862: f64, t7869: f64, t196: f64, t197: f64, t5528: f64, t2035: f64, t7313: f64, t7898: f64, t1032: f64, t1892: f64, t1955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27800, t27802, t27806, t27810, t27817, t27821) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1854(t27384, t27799, t1113, t1583, t33, t4537, t1711, t775, t890, t1940, t1963, t2403, t25206, t25440, t27158, t27364, t27368, t27382, t27407, t27764, t27770, t27773, t27777, t27793, t7087, t7091, t7200, t7207, t7783, t7862, t7869);
        let (t27833, t27834, t27835, t27836, t27837) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1855(t196, t197, t5528, t2035, t7313, t7898, t1032, t1892, t1955);
    (t27800, t27802, t27806, t27810, t27817, t27821, t27833, t27834, t27835, t27836, t27837)
}
