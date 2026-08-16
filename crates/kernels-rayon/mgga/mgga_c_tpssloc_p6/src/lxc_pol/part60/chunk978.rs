//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 978/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk978(t22685: f64, t28191: f64, t31137: f64, t120317: f64, t1842: f64, t1992: f64, t22635: f64, t1985: f64, t28232: f64, t120544: f64, t6888: f64, t7691: f64) -> (f64, f64, f64, f64) {
    let t127176 = 0.9869604401089358619e-1_f64 * t22685 * t31137 * t28191;
    let t127180 = 0.6579736267392905746e-1_f64 * t1992 * t22635 * t120317 * t1842;
    let t127183 = 0.3289868133696452873e-1_f64 * t1985 * t31137 * t28232;
    let t127187 = 0.6579736267392905746e-1_f64 * t6888 * t120544 * t7691;
    (t127176, t127180, t127183, t127187)
}
