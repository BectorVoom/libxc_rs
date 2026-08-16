//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1299/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1299(t102166: f64, t102170: f64, t102205: f64, t102209: f64, t102221: f64, t102237: f64, t12605: f64, t1307: f64, t1600: f64, t21020: f64, t27567: f64, t27583: f64, t27584: f64, t27607: f64, t28701: f64, t28760: f64, t29591: f64, t29595: f64, t4440: f64, t77072: f64, t7978: f64, t7979: f64, t99301: f64) -> f64 {
    let t102239 = 0.51588271604938271604e-3_f64 * t102205 + 0.46336805555555555556e-3_f64 * t99301 * t28760 - 0.23168402777777777778e-3_f64 * t27583 * t12605 * t102209 * t1307 + 0.23168402777777777778e-3_f64 * t99301 * t28701 + 0.13901041666666666667e-2_f64 * t27583 * t102166 + 0.13901041666666666667e-2_f64 * t27583 * t102170 - 0.46377350260416666667e-4_f64 * t27567 * t102221 + 0.11584201388888888889e-3_f64 * t27583 * t4440 * t27584 * t77072 - 0.11584201388888888889e-3_f64 * t27607 * t29591 - 0.11584201388888888889e-3_f64 * t7978 * t1600 * t7979 * t21020 - 0.15445601851851851852e-3_f64 * t27607 * t29595 - 0.11607361111111111111e-2_f64 * t102237;
    t102239
}
