//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta558(t1032: f64, t5710: f64, t1426: f64, t7063: f64, t1892: f64, t25877: f64, t1955: f64, t14066: f64, t1883: f64, t4077: f64, t25981: f64, t5677: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97961, t97962, t98040, t98041, t98050, t98053, t98062, t98108) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1900(t1032, t5710, t1426, t7063, t1892, t25877, t1955, t14066, t1883, t4077, t25981, t5677, t820, t844);
    (t97961, t97962, t98040, t98041, t98050, t98053, t98062, t98108)
}
