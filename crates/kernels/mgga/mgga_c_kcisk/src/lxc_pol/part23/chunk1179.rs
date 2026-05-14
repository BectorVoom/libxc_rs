//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1179/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1179<F: Float>(t9511: F, t9528: F, t1597: F, t4375: F, t4376: F, t1586: F, t1589: F, t4497: F, t32185: F, t2737: F, t2740: F, t32155: F, t32157: F, t32171: F, t32187: F, t32199: F, t32201: F, t32205: F, t32209: F, t32213: F, t32220: F, t32385: F, t9519: F, t9524: F, t9544: F) -> (F, F, F, F, F, F, F) {
    let t32480 = t9511 * t9528;
    let t32484 = t4375 * t1597 * t4376;
    let t32485 = t1586 * t32484;
    let t32497 = t1589 * t1597 * t4497;
    let t32498 = t1586 * t32497;
    let t32502 = 0.38691203703703703703e-3 * t32185;
    let t32510 = 0.27777777777777777778e-1 * t32480 * t2740 - 0.10416666666666666667e-1 * t2737 * t32485 - 0.23214722222222222222e-2 * t32155 + 0.15476481481481481481e-2 * t32157 + 0.52083333333333333333e-2 * t2737 * t32385 + 0.10416666666666666667e-1 * t9524 * t9544 + 0.10416666666666666667e-1 * t9524 * t9519 + 0.52083333333333333333e-2 * t2737 * t32498 + 0.17024129629629629629e-1 * t32171 - t32502 - 0.61905925925925925925e-2 * t32187 - 0.23214722222222222222e-2 * t32199 + 0.15476481481481481481e-2 * t32201 + 0.23214722222222222222e-2 * t32205 + 0.11607361111111111111e-2 * t32209 + 0.19345601851851851852e-2 * t32213 - 0.92858888888888888886e-2 * t32220;
    (t32480, t32484, t32485, t32497, t32498, t32502, t32510)
}
