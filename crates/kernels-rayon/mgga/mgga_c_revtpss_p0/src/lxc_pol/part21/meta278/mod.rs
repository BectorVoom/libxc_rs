//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1506;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta278(t4066: f64, t72: f64, t1432: f64, t686: f64, t136: f64, t1419: f64, t2457: f64, t3964: f64, t225: f64, t9646: f64, t1428: f64, t22: f64, t2452: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10103, t10105, t10107, t10109, t10111) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1506(t4066, t72, t1432, t686, t136, t1419, t2457, t3964, t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1507(t10111, t1428, t22, t2452);
    (t10103, t10105, t10107, t10109, t10111, t10114, t10115)
}
