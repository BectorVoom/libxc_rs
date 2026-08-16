//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 898/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk898(t23528: f64, t354: f64, t3053: f64, t6765: f64, t3127: f64, t3037: f64, t3033: f64, t6753: f64, t1004: f64, t6764: f64, t1036: f64, t6750: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23529 = t354 * t23528;
    let t23533 = t6765 * t3053;
    let t23535 = t3127 * sigma0;
    let t23536 = t23535 * t3037;
    let t23537 = t3033 * t23536;
    let t23540 = t6753 * t3037;
    let t23541 = t3033 * t23540;
    let t23544 = t1004 * t6764;
    let t23554 = t6750 * t1036;
    (t23529, t23533, t23535, t23537, t23541, t23544, t23554)
}
