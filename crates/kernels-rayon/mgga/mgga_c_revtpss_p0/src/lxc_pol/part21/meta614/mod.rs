//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta614(t10467: f64, t705: f64, t39875: f64, t39894: f64, t9371: f64, t760: f64, t39960: f64, t39963: f64, t2523: f64, t9372: f64, t2258: f64, t4401: f64, t606: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40158, t40165, t40167, t40169, t40171, t40172, t40178) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2366(t10467, t705, t39875, t39894, t9371, t760, t39960, t39963, t2523, t9372, t2258, t4401, t606, t749);
    (t40158, t40165, t40167, t40169, t40171, t40172, t40178)
}
