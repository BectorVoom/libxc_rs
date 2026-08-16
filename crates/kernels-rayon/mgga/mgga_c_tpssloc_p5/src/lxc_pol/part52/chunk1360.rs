//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1360/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1360(t20173: f64, t33193: f64, t3941: f64, t4072: f64, t8326: f64, t7015: f64, t86647: f64, t16524: f64, t31285: f64, t16521: f64, t12524: f64, t33188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120800 = 27.0_f64 * t20173 * t33193;
    let t120803 = 27.0_f64 * t3941 * t8326 * t4072;
    let t120804 = t86647 * t7015;
    let t120807 = 27.0_f64 * t16524 * t31285;
    let t120809 = 0.135e2_f64 * t16521 * t8326;
    let t120811 = 54.0_f64 * t12524 * t33188;
    (t120800, t120803, t120804, t120807, t120809, t120811)
}
