//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 963/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk963(t12734: f64, t8326: f64, t12739: f64, t1388: f64, t6995: f64, t31283: f64, t16535: f64, t2363: f64, t3941: f64, t12524: f64, t31285: f64, t12521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114413 = 4.0_f64 * t12734 * t8326;
    let t114415 = 2.0_f64 * t12739 * t8326;
    let t114422 = t1388 * t6995;
    let t114456 = 27.0_f64 * t31283;
    let t114472 = 27.0_f64 * t16535 * t8326;
    let t114483 = 27.0_f64 * t3941 * t8326 * t2363;
    let t114489 = 54.0_f64 * t12524 * t31285;
    let t114494 = 0.135e2_f64 * t12521 * t8326;
    (t114413, t114415, t114422, t114456, t114472, t114483, t114489, t114494)
}
