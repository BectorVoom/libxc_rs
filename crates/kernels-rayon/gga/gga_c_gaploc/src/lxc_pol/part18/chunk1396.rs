//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1396/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1396(t10396: f64, t20565: f64, t31586: f64, t4820: f64, t6824: f64, t31591: f64, t10399: f64, t21272: f64, t2478: f64, t2792: f64, t6576: f64, t7047: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34706 = 0.15889106645266856297e0_f64 * t20565 * t10396;
    let t34709 = 0.15889106645266856297e0_f64 * t6824 * t4820 * t31586;
    let t34712 = 0.15889106645266856297e0_f64 * t6824 * t4820 * t31591;
    let t34713 = t21272 * t10399;
    let t34714 = 0.38342925953920749676e0_f64 * t34713;
    let t34716 = t6576 * t2792 * t2478;
    let t34717 = 0.38342925953920749676e0_f64 * t34716;
    let t34719 = t6576 * t993 * t7047;
    (t34706, t34709, t34712, t34714, t34717, t34719)
}
