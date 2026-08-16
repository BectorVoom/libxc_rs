//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta485(t1873: f64, t26004: f64, t5690: f64, t7252: f64, t1398: f64, t1903: f64, t543: f64, t1955: f64, t5710: f64, t1513: f64, t25823: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t27955, t27957, t27972, t28008, t28034, t28036) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1768(t1873, t26004, t5690, t7252, t1398, t1903, t543, t1955, t5710, t1513, t25823, t665);
    (t27955, t27957, t27972, t28008, t28034, t28036)
}
