//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1288/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1288(t16524: f64, t31285: f64, t16521: f64, t8326: f64, t12524: f64, t33188: f64, t26135: f64, t7010: f64, t120758: f64, t120786: f64, t120788: f64, t120789: f64, t120792: f64, t120793: f64, t120795: f64, t120800: f64, t120803: f64, t120804: f64, t31284: f64, t33195: f64, t577: f64, t8508: f64) -> f64 {
    let t120807 = 27.0_f64 * t16524 * t31285;
    let t120809 = 0.135e2_f64 * t16521 * t8326;
    let t120811 = 54.0_f64 * t12524 * t33188;
    let t120812 = t7010 * t26135;
    let t120814 = t31284 + t8508 + t120786 + t120788 + 54.0_f64 * t120789 + t33195 + t120792 + 27.0_f64 * t120793 + 54.0_f64 * t120795 + 0.45e1_f64 * t120758 * t577 + t120800 + t120803 + 54.0_f64 * t120804 + t120807 + t120809 + t120811 + 27.0_f64 * t120812;
    t120814
}
