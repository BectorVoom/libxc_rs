//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1306/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1306(t1250: f64, t71731: f64, t27974: f64, t8030: f64, t101028: f64, t101208: f64, t27958: f64, t2894: f64, t71387: f64, t7693: f64, t7703: f64, t7704: f64, t95524: f64, t96412: f64, t96418: f64, t96428: f64, t96449: f64, t96451: f64) -> f64 {
    let t101554 = t71731 * t1250;
    let t101567 = t8030 * t27974;
    let t101569 = -0.7369753086419753086e-3_f64 * t96412 + 0.92754700520833333333e-4_f64 * t101554 * t7693 + 0.6183646701388888889e-4_f64 * t95524 * t27958 - 0.46336805555555555557e-3_f64 * t7703 * t101208 + 0.23168402777777777778e-3_f64 * t7703 * t2894 * t7704 * t71387 + 0.23168402777777777778e-3_f64 * t7703 * t101028 + 0.46336805555555555557e-3_f64 * t101567 - t96418 + t96428 + t96449 - t96451;
    t101569
}
