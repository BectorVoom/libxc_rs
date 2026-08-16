//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta465(t1923: f64, t26205: f64, t122: f64, t2097: f64, t72: f64, t25900: f64, t25904: f64, t3916: f64, t25895: f64, t3920: f64, t7496: f64, t2098: f64, t2453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26207, t26230, t26231, t26232, t26234, t26235, t26238, t26249) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1691(t1923, t26205, t122, t2097, t72, t25900, t25904, t3916, t25895, t3920, t7496, t2098, t2453);
    (t26207, t26230, t26231, t26232, t26234, t26235, t26238, t26249)
}
