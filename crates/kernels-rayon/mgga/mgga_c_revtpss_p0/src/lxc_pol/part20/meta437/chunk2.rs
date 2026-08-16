//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1650/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1650(t43762: f64, t43769: f64, t43771: f64, t43773: f64, t43779: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43791: f64, t43795: f64, t43799: f64, t43802: f64, t43804: f64) -> f64 {
    let t45103 = -0.12349037037037037037e0_f64 * t43762 - 0.10805407407407407407e0_f64 * t43769 - 0.12349037037037037037e1_f64 * t43771 + 0.55570666666666666668e0_f64 * t43773 + 0.55570666666666666666e0_f64 * t43779 + 0.69463333333333333334e0_f64 * t43781 + 0.13892666666666666667e1_f64 * t43783 - 0.27785333333333333333e0_f64 * t43785 - 0.166712e1_f64 * t43787 - 0.125034e1_f64 * t43791 + 0.250068e1_f64 * t43795 + 0.104195e0_f64 * t43799 + 0.158837625e2_f64 * t43802 - 0.705945e1_f64 * t43804;
    t45103
}
