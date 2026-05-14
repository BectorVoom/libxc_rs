//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1382/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1382<F: Float>(t115849: F, t1390: F, t2331: F, t5626: F, t115666: F, t115667: F, t33909: F, t115711: F, t3532: F, t109633: F, t115496: F, t115504: F, t115515: F, t115531: F, t115858: F, t119141: F, t119144: F, t119154: F, t119162: F, t32417: F, t32474: F, t33928: F, t34945: F, t9869: F) -> (F, F, F, F) {
    let t120348 = t115849 * t2331 * t1390 * t5626;
    let t120352 = t115666 * t115667 * t33909;
    let t120357 = t115711 * t2331 * t3532 * t5626;
    let t120367 = t115496 + t115504 - 0.30952962962962962963e-2 * t119141 - 0.23214722222222222222e-2 * t119144 - 0.69444444444444444444e-2 * t115515 - t115531 + 0.11607361111111111111e-2 * t119154 - 0.26805555555555555556e-2 * t109633 * t120348 - 0.77602083333333333335e-3 * t115858 * t120352 + 0.17870370370370370371e-2 * t109633 * t120357 + 0.15476481481481481481e-2 * t119162 - 0.60312500000000000001e-2 * t32474 * t34945 - 0.60312500000000000001e-2 * t32417 * t34945 + 0.10416666666666666667e-1 * t33928 * t9869;
    (t120348, t120352, t120357, t120367)
}
