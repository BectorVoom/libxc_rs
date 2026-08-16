//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1458/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1458(t12461: f64, t8639: f64, t26161: f64, t26163: f64, t119853: f64, t22574: f64, t24432: f64, t671: f64, t8518: f64, t1983: f64, t31035: f64, t7940: f64) -> (f64, f64, f64, f64) {
    let t122675 = t8639 * t12461;
    let t122678 = 2.0_f64 * t26161 * t122675 * t26163;
    let t122681 = 3.0_f64 * t22574 * t24432 * t119853;
    let t122685 = t8518 * t671;
    let t122692 = t1983 * t7940 * t31035;
    (t122678, t122681, t122685, t122692)
}
