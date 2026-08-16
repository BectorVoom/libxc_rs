//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta440(t1340: f64, t40196: f64, t40192: f64, t40113: f64, t40169: f64, t40135: f64, t3869: f64, t39739: f64, t39430: f64, t39742: f64, t39440: f64, t39532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47122, t47124, t47131) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1396(t1340, t40196, t40192, t40113, t40169, t40135, t3869, t39739, t39430, t39742, t39440, t39532);
    (t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47122, t47124, t47131)
}
