//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1428/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1428(t1442: f64, t17940: f64, t3146: f64, t55922: f64, t894: f64, t1133: f64, t17674: f64, t17677: f64, t17714: f64, t17720: f64, t17864: f64, t35643: f64, t4310: f64, t4369: f64, t46810: f64, t47069: f64, t47138: f64, t47149: f64, t54520: f64, t54523: f64, t54527: f64, t54541: f64, t54589: f64, t54613: f64, t54616: f64, t54619: f64, t54622: f64, t8973: f64) -> (f64, f64, f64) {
    let t59618 = t17940 * t1442;
    let t59637 = t894 * t3146 * t55922;
    let t59643 = 0.95929744112718128262e1_f64 * t54520 + t54523 / 54.0_f64 + 0.47242254414539272975e4_f64 * t54527 - 0.77272546575900069819e-1_f64 * t54541 - 0.12209704640613106892e2_f64 * t47069 - 11.0_f64 / 81.0_f64 * t47138 + 0.73258227843678641352e2_f64 * t8973 * t46810 * t59618 - 0.47333755318775392234e-1_f64 * t47149 + 0.15454509315180013964e0_f64 * t54589 - 0.28345352648723563785e5_f64 * t54613 + 0.28977204965962526181e-1_f64 * t54616 + 0.94667510637550784468e-1_f64 * t54619 + 0.48295341609937543636e-2_f64 * t54622 + 2.0_f64 / 9.0_f64 * t4310 * t17674 - 4.0_f64 / 27.0_f64 * t4310 * t17677 - 0.3863627328795003491e-1_f64 * t4369 * t17720 + 0.3863627328795003491e0_f64 * t4369 * t17864 + 0.90553765518632894319e-2_f64 * t1133 * t59637 - 0.23181763972770020946e0_f64 * t4369 * t17714 - 10.0_f64 / 243.0_f64 * t35643;
    (t59618, t59637, t59643)
}
