//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta439(t39768: f64, t47065: f64, t190: f64, t22: f64, t519: f64, t39762: f64, t1317: f64, t9545: f64, t1340: f64, t40129: f64, t40182: f64, t39821: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t47067, t47070, t47072, t47074, t47076, t47084, t47086) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1395(t39768, t47065, t190, t22, t519, t39762, t1317, t9545, t1340, t40129, t40182, t39821);
    (t47067, t47070, t47072, t47074, t47076, t47084, t47086)
}
