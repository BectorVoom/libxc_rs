//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1699;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta455(t114: f64, t26028: f64, t3940: f64, t3926: f64, t7264: f64, t25304: f64, t7283: f64, t25949: f64, t786: f64, t1426: f64, t3999: f64, t25821: f64, t25824: f64, t25827: f64, t25829: f64, t508: f64, t2106: f64, t530: f64, t25865: f64, t6977: f64, t7348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26029, t26031, t26069, t26072, t26079, t26148, t26153) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1699(t114, t26028, t3940, t3926, t7264, t25304, t7283, t25949, t786, t1426, t3999, t25821, t25824, t25827, t25829);
        let (t26154, t26162, t26169) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1700(t26153, t508, t2106, t530, t25865, t6977, t7348);
    (t26029, t26031, t26069, t26072, t26079, t26148, t26153, t26154, t26162, t26169)
}
