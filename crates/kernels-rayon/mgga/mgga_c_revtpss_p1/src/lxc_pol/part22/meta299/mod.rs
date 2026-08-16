//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1729;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta299(t1386: f64, t820: f64, t844: f64, t3940: f64, t221: f64, t3924: f64, t4019: f64, t4018: f64, t3930: f64, t4059: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t9962 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1729(t1386, t820, t844);
        let (t9963, t9970, t9971, t9973, t9976) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1730(t3940, t9962, t221, t3924, t4019, t4018, t3930, t4059, t1386, t2482, t596);
    (t9962, t9963, t9970, t9971, t9973, t9976)
}
