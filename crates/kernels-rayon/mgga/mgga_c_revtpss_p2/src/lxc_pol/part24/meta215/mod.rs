//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta215(t2922: f64, t913: f64, t275: f64, t290: f64, t2925: f64, t2966: f64, t307: f64, t302: f64, t11132: f64, t11337: f64, t944: f64, t2969: f64, t310: f64, t3010: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11384, t11385, t11387, t11408, t11409, t11422, t11423, t11449, t11450, t11452) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk959(t2922, t913, t275, t290, t2925, t2966, t307, t302, t11132, t11337, t944, t2969, t310);
        let t11465 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk960(t3010, t320);
    (t11384, t11385, t11387, t11408, t11409, t11422, t11423, t11449, t11450, t11452, t11465)
}
