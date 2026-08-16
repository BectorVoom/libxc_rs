//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1199/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1199(t3053: f64, t6765: f64, t3127: f64, t3037: f64, t3033: f64, t6753: f64, t1004: f64, t6764: f64, t3014: f64, t343: f64, t6734: f64, t6758: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23533 = t6765 * t3053;
    let t23535 = t3127 * sigma0;
    let t23536 = t23535 * t3037;
    let t23537 = t3033 * t23536;
    let t23540 = t6753 * t3037;
    let t23541 = t3033 * t23540;
    let t23544 = t1004 * t6764;
    let t23547 = t3014 * t343;
    let t23548 = t23547 * t6734;
    let t23551 = t1004 * t6758;
    (t23533, t23535, t23536, t23537, t23540, t23541, t23544, t23547, t23548, t23551)
}
