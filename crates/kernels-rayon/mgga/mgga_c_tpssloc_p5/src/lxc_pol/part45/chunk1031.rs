//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1031/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1031(t22949: f64, t8607: f64, t1983: f64, t22584: f64, t31758: f64, t31035: f64, t7217: f64, t22597: f64, t12734: f64, t8533: f64, t2314: f64, t31772: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115690 = t8607 * t22949;
    let t115695 = 3.0_f64 * t1983 * t31758 * t22584;
    let t115698 = 2.0_f64 * t1983 * t7217 * t31035;
    let t115700 = 6.0_f64 * t8607 * t22597;
    let t115702 = 4.0_f64 * t12734 * t8533;
    let t115704 = 4.0_f64 * t2314 * t31772;
    (t115690, t115695, t115698, t115700, t115702, t115704)
}
