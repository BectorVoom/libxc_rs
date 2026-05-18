//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1428/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1428<F: Float>(t1442: F, t17940: F, t3146: F, t55922: F, t894: F, t1133: F, t17674: F, t17677: F, t17714: F, t17720: F, t17864: F, t35643: F, t4310: F, t4369: F, t46810: F, t47069: F, t47138: F, t47149: F, t54520: F, t54523: F, t54527: F, t54541: F, t54589: F, t54613: F, t54616: F, t54619: F, t54622: F, t8973: F) -> (F, F, F) {
    let t59618 = t17940 * t1442;
    let t59637 = t894 * t3146 * t55922;
    let t59643 = F::new(0.95929744112718128262e1) * t54520 + t54523 / F::new(54.0) + F::new(0.47242254414539272975e4) * t54527 - F::new(0.77272546575900069819e-1) * t54541 - F::new(0.12209704640613106892e2) * t47069 - F::new(11.0) / F::new(81.0) * t47138 + F::new(0.73258227843678641352e2) * t8973 * t46810 * t59618 - F::new(0.47333755318775392234e-1) * t47149 + F::new(0.15454509315180013964e0) * t54589 - F::new(0.28345352648723563785e5) * t54613 + F::new(0.28977204965962526181e-1) * t54616 + F::new(0.94667510637550784468e-1) * t54619 + F::new(0.48295341609937543636e-2) * t54622 + F::new(2.0) / F::new(9.0) * t4310 * t17674 - F::new(4.0) / F::new(27.0) * t4310 * t17677 - F::new(0.3863627328795003491e-1) * t4369 * t17720 + F::new(0.3863627328795003491e0) * t4369 * t17864 + F::new(0.90553765518632894319e-2) * t1133 * t59637 - F::new(0.23181763972770020946e0) * t4369 * t17714 - F::new(10.0) / F::new(243.0) * t35643;
    (t59618, t59637, t59643)
}
