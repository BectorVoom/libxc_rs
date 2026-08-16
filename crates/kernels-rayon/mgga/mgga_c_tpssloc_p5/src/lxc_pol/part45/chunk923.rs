//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 923/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk923(t30635: f64, t6579: f64, t1888: f64, t23270: f64, t25169: f64, t2719: f64, t22986: f64, t30623: f64, t82159: f64, t23185: f64, t30634: f64, t82074: f64) -> (f64, f64, f64, f64) {
    let t112686 = t6579 * t30635;
    let t112687 = 0.15352717957250113407e0_f64 * t112686;
    let t112697 = 0.9869604401089358619e-1_f64 * t1888 * t23270 * t25169 * t2719;
    let t112700 = 0.6579736267392905746e-1_f64 * t22986 * t82159 * t30623;
    let t112702 = t23185 * t82074 * t30634;
    (t112687, t112697, t112700, t112702)
}
