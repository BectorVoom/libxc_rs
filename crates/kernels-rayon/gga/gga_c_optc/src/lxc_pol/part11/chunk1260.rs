//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1260/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1260(t24619: f64, t55901: f64, t55912: f64, t865: f64, t2648: f64, t55906: f64, t894: f64, t10917: f64, t10976: f64, t1235: f64, t1325: f64, t16632: f64, t2640: f64, t2643: f64, t31719: f64, t31765: f64, t322: f64, t40489: f64, t40677: f64, t4961: f64, t49850: f64, t50955: f64, t50985: f64, t50994: f64, t7491: f64, t8134: f64, t8209: f64, t862: f64, t893: f64) -> (f64, f64, f64, f64) {
    let t56844 = t24619 * t55901;
    let t56848 = t865 * t55912;
    let t56862 = t894 * t2648 * t55906;
    let t56865 = 0.28345352648723563784e5_f64 * t8134 * t49850 * t8209 * t1325 - 0.94667510637550784466e0_f64 * t2640 * t10917 * t2643 * t16632 + 0.21464596271083352727e-2_f64 * t31719 - t40677 / 216.0_f64 + 35.0_f64 / 972.0_f64 * t862 * t322 * t56844 + t862 * t322 * t56848 / 288.0_f64 + 0.73258227843678641352e2_f64 * t7491 * t10976 * t40489 * t1235 * t4961 + 0.24147670804968771818e-1_f64 * t50955 + 0.42074449172244793097e-1_f64 * t31765 - 0.28977204965962526181e-1_f64 * t50985 + 0.94667510637550784468e-1_f64 * t50994 - 0.10866451862235947318e-1_f64 * t893 * t56862;
    (t56844, t56848, t56862, t56865)
}
