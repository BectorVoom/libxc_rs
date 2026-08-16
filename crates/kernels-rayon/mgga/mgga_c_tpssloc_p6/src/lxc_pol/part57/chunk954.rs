//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 954/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk954(t22986: f64, t23270: f64, t30622: f64, t5544: f64, t118649: f64, t118532: f64, t32844: f64, t16891: f64, t232: f64, t30714: f64, t4180: f64, t112792: f64, t16839: f64, t2632: f64) -> (f64, f64, f64, f64, f64) {
    let t126290 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t30622 * t5544;
    let t126291 = 0.15352717957250113407e0_f64 * t118649;
    let t126294 = t118532 * t32844;
    let t126298 = t30714 * t4180 * t16891 * t232;
    let t126302 = t112792 * t4180 * t16839 * t2632;
    (t126290, t126291, t126294, t126298, t126302)
}
