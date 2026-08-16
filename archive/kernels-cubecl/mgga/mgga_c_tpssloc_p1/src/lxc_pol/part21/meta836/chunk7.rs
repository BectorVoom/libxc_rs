//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2977/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2977<F: Float>(t1041: F, t10868: F, t248: F, t5681: F, t10949: F, t14080: F, t14172: F, t14187: F, t1622: F, t17643: F, t17734: F, t17972: F, t3117: F, t4582: F, t4583: F, t4588: F, t4636: F, t49716: F, t49721: F, t49732: F, t49740: F, t50334: F, t55662: F, t55666: F, t62044: F) -> F {
    let t62137 = t1041 * t248 * t10868 * t5681;
    let t62145 = t10949 * t17734 / F::cast_from(384.0_f64) + t3117 * t17972 / F::cast_from(384.0_f64) + t49716 / F::cast_from(576.0_f64) + t49721 / F::cast_from(2304.0_f64) + t49732 / F::cast_from(72.0_f64) - t1041 * t4582 * t4583 * t55666 / F::cast_from(1152.0_f64) - t1041 * t4582 * t4583 * t55662 / F::cast_from(2304.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1041 * t4582 * t14172 * t62044 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3117 * t17643 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1041 * t4582 * t4588 * t55666 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1041 * t4582 * t4588 * t55662 + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1041 * t4582 * t14187 * t62044 + t62137 / F::cast_from(10368.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t50334 * t1622 - t49740 * t1622 / F::cast_from(216.0_f64) - t14080 * t4636 / F::cast_from(216.0_f64);
    t62145
}
