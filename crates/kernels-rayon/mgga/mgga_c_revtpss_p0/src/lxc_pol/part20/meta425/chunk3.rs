//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1596/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1596(t43762: f64, t43769: f64, t43771: f64, t43773: f64, t43779: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43791: f64, t43795: f64, t43799: f64, t43802: f64, t43804: f64) -> f64 {
    let t44036 = -0.97370864197530864196e-1_f64 * t43762 - 0.85199506172839506175e-1_f64 * t43769 - 0.97370864197530864199e0_f64 * t43771 + 0.43816888888888888888e0_f64 * t43773 + 0.43816888888888888889e0_f64 * t43779 + 0.54771111111111111111e0_f64 * t43781 + 0.10954222222222222222e1_f64 * t43783 - 0.21908444444444444444e0_f64 * t43785 - 0.13145066666666666666e1_f64 * t43787 - 0.98587999999999999998e0_f64 * t43791 + 0.197176e1_f64 * t43795 + 0.82156666666666666667e-1_f64 * t43799 + 0.85451625e1_f64 * t43802 - 0.379785e1_f64 * t43804;
    t44036
}
