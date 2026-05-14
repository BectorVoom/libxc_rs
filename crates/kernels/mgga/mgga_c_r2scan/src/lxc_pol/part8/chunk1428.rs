//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1428/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1428<F: Float>(t10359: F, t1632: F, t549: F, t551: F, t10132: F, t20705: F, t2526: F, t2640: F, t2656: F, t30850: F, t31037: F, t31040: F, t31047: F, t31051: F, t31057: F, t31061: F, t31066: F, t3190: F, t3218: F, t32266: F, t32792: F, t506: F, t527: F, t529: F, t552: F, t6218: F, t6522: F, t7245: F) -> (F,) {
    let t34621 = t549 * t551 * t1632 * t10359;
    let t34628 = 0.39006997830244208535e0 * t31037 * t2656 - 0.7801399566048841707e0 * t6218 * t551 * t552 * t3190 * t2526 - 0.15602799132097683414e1 * t20705 * t10132 + 0.69345773920434148506e0 * t31040 + 0.15602799132097683414e1 * t30850 * t2640 - 0.19756347548806534796e1 * t6522 * t529 * t506 * t32792 - 0.54878743191129263322e-1 * t527 * t529 * t506 * t32266 - 0.13002332610081402845e0 * t7245 * t3218 + 0.11557628986739024751e0 * t34621 + 0.82318114786693894983e-2 * t31047 - 0.52396431978519890151e-1 * t31051 - 0.12225834128321307702e1 * t31057 - 0.17465477326173296717e-1 * t31061 - 0.2095857279140795606e0 * t31066;
    (t34628,)
}
