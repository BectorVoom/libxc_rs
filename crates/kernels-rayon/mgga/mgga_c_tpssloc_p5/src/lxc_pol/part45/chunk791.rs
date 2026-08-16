//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 791/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk791(t23604: f64, t3187: f64, t23603: f64, t3192: f64, t6800: f64, t6799: f64, t225: f64, t6733: f64, t6786: f64, t1949: f64, t2966: f64, t1920: f64) -> (f64, f64, f64, f64) {
    let t23605 = t3187 * t23604;
    let t23606 = t23603 * t23605;
    let t23609 = t3192 * t6800;
    let t23610 = t6799 * t23609;
    let t23613 = t6733 * t225;
    let t23614 = t23613 * t6786;
    let t23617 = t2966 * t1949;
    let t23619 = 0.18277045187202515961e-2_f64 * t1920 * t23617;
    (t23606, t23610, t23614, t23619)
}
