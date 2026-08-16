//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta458(t25895: f64, t26234: f64, t3920: f64, t7496: f64, t1398: f64, t543: f64, t7506: f64, t7301: f64, t2097: f64, t4056: f64, t2098: f64, t2453: f64) -> (f64, f64, f64, f64, f64) {
        let (t26235, t26238, t26241, t26246, t26249) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1707(t25895, t26234, t3920, t7496, t1398, t543, t7506, t7301, t2097, t4056, t2098, t2453);
    (t26235, t26238, t26241, t26246, t26249)
}
