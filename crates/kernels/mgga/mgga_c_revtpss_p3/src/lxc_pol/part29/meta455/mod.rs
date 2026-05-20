//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1699;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta455<F: Float>(t114: F, t26028: F, t3940: F, t3926: F, t7264: F, t25304: F, t7283: F, t25949: F, t786: F, t1426: F, t3999: F, t25821: F, t25824: F, t25827: F, t25829: F, t508: F, t2106: F, t530: F, t25865: F, t6977: F, t7348: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26029, t26031, t26069, t26072, t26079, t26148, t26153) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1699::<F>(t114, t26028, t3940, t3926, t7264, t25304, t7283, t25949, t786, t1426, t3999, t25821, t25824, t25827, t25829);
        let (t26154, t26162, t26169) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1700::<F>(t26153, t508, t2106, t530, t25865, t6977, t7348);
    (t26029, t26031, t26069, t26072, t26079, t26148, t26153, t26154, t26162, t26169)
}
