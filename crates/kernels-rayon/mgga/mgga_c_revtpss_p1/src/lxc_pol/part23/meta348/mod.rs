//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1652;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta348(t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64, t2798: f64, t1568: f64, t2783: f64, t786: f64, t2801: f64, t233: f64, t4469: f64, t869: f64, t689: f64, t2435: f64, t4519: f64, t1558: f64, t2723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14557, t14558, t14563, t14564, t14567, t14568) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1652(t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783, t786);
        let (t14570, t14574, t14575, t14577, t14581, t14586) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1653(t14568, t2801, t233, t4469, t869, t689, t2435, t4519, t1558, t2723);
    (t14557, t14558, t14563, t14564, t14567, t14568, t14570, t14574, t14575, t14577, t14581, t14586)
}
