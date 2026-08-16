//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1088/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1088(t3791: f64, t562: f64, t10: f64, t2229: f64, t116: f64, t117: f64, t556: f64, t252: f64, t2631: f64, t243: f64, t828: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22740 = t562 * t3791;
    let t22811 = t2229 * t10;
    let t22815 = t117 * t116;
    let t22842 = t556 * t556;
    let t22843 = 1.0_f64 / t22842;
    let t22997 = t252 * t2631;
    let t23075 = t243 * t243;
    let t23076 = 1.0_f64 / t23075;
    let t23175 = t852 * t828;
    (t22740, t22811, t22815, t22843, t22997, t23076, t23175)
}
