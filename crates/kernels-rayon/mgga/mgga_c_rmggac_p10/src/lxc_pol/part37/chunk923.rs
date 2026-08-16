//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 923/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk923(t76700: f64, t15450: f64, t7244: f64, t1971: f64, t495: f64, t7230: f64, t875: f64, t9551: f64, t15626: f64, t34884: f64, t3352: f64, t515: f64, t9523: f64) -> (f64, f64, f64, f64, f64) {
    let t76701 = 0.25538759935978703639e-4_f64 * t76700;
    let t76702 = t7244 * t15450;
    let t76703 = 0.99317399751028291929e-5_f64 * t76702;
    let t76707 = t7230 * t1971 * t875 * t9551 * t495;
    let t76708 = 0.1064114997332445985e-4_f64 * t76707;
    let t76712 = t34884 * t15626;
    let t76713 = 0.12414674968878536491e-4_f64 * t76712;
    let t76717 = t7230 * t3352 * t515 * t9523 * t495;
    (t76701, t76703, t76708, t76713, t76717)
}
