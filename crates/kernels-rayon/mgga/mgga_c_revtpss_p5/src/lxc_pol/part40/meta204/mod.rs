//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk836;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta204(t231: f64, t2783: f64, t4494: f64, t2782: f64, t1559: f64, t72: f64, t686: f64, t2798: f64, t225: f64, t2718: f64, t213: f64, t1568: f64, t233: f64, t869: f64, t689: f64, t874: f64, t822: f64, t234: f64, t2776: f64, t2780: f64, t2787: f64, t2791: f64, t2796: f64, t2802: f64, t2806: f64, t2810: f64, t2815: f64, t4366: f64, t4424: f64, t4469: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4496, t4497, t4499, t4500, t4501, t4503, t4504, t4514) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk836(t231, t2783, t4494, t2782, t1559, t72, t686, t2798, t225, t2718, t213);
        let (t4518, t4519, t4522, t4526, t4533) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk837(t1568, t233, t869, t689, t72, t686, t874, t822, t1559, t213, t234, t2776, t2780, t2787, t2791, t2796, t2802, t2806, t2810, t2815, t4366, t4424, t4469, t4494, t4497, t4501, t4504, t4514, t820, t837, t879);
    (t4496, t4499, t4500, t4503, t4504, t4514, t4518, t4519, t4522, t4526, t4533)
}
