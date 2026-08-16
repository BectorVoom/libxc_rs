//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1424;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta456(t14473: f64, t9303: f64, t1593: f64, t9292: f64, t1606: f64, t11384: f64, t1596: f64, t11465: f64, t1626: f64, t11298: f64, t11506: f64, t11408: f64, t1614: f64, t11449: f64, t11199: f64, t1646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51733, t51978) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1424(t14473, t9303, t1593, t9292);
        let (t52128, t52224, t52443, t52508, t52642, t52812, t52825, t53014) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1425(t1606, t9303, t11384, t1596, t11465, t1626, t11298, t11506, t11408, t1614, t11449, t11199, t1646);
    (t51733, t51978, t52128, t52224, t52443, t52508, t52642, t52812, t52825, t53014)
}
