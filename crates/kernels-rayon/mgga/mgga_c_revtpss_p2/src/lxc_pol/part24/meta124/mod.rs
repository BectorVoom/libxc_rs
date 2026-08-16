//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk662;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk663;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta124(t213: f64, t4503: f64, t2783: f64, t1568: f64, t233: f64, t869: f64, t689: f64, t72: f64, t686: f64, t874: f64, t822: f64, t198: f64, t205: f64, t1583: f64, t892: f64, t1593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4504, t4514, t4518, t4519, t4520, t4522, t4524, t4526, t4541) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk662(t213, t4503, t2783, t1568, t233, t869, t689, t72, t686, t874, t822, t198, t205);
        let t4546 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk663(t1583, t892);
        let t4571 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk664(t1593, t689);
    (t4504, t4514, t4518, t4519, t4520, t4522, t4524, t4526, t4541, t4546, t4571)
}
