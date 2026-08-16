//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1270/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1270(t22986: f64, t23270: f64, t30633: f64, t98960: f64, t112899: f64, t25038: f64, t25040: f64, t1888: f64, t32862: f64, t82159: f64, t112667: f64, t112673: f64) -> (f64, f64, f64, f64, f64) {
    let t118488 = 0.6579736267392905746e-1_f64 * t22986 * t23270 * t30633 * t98960;
    let t118491 = 0.9869604401089358619e-1_f64 * t25038 * t112899 * t25040;
    let t118498 = 0.3289868133696452873e-1_f64 * t1888 * t82159 * t32862;
    let t118499 = 0.38381794893125283518e-1_f64 * t112667;
    let t118500 = 0.38381794893125283518e-1_f64 * t112673;
    (t118488, t118491, t118498, t118499, t118500)
}
