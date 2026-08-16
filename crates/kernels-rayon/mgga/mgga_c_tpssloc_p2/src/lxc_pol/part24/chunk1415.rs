//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1415/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1415(t22561: f64, t4034: f64, t3652: f64, t652: f64, t6534: f64, t1874: f64, t45602: f64, t6525: f64, t9348: f64, t15904: f64, t22574: f64, t31035: f64) -> (f64, f64, f64, f64, f64) {
    let t83674 = 12.0_f64 * t4034 * t22561;
    let t83677 = 6.0_f64 * t652 * t3652 * t6534;
    let t83679 = 6.0_f64 * t45602 * t1874;
    let t83681 = 6.0_f64 * t9348 * t6525;
    let t83684 = 18.0_f64 * t22574 * t31035 * t15904;
    (t83674, t83677, t83679, t83681, t83684)
}
