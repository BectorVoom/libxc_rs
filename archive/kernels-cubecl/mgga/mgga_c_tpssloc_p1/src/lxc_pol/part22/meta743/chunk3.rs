//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2467/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2467<F: Float>(t1041: F, t21134: F, t248: F, t3051: F, t14508: F, t17667: F, t14085: F, t1622: F, t17962: F, t21405: F, t21580: F, t21597: F, t3109: F, t3117: F, t42354: F, t4641: F, t48431: F, t50302: F, t5857: F, t5875: F, t61677: F, t61695: F) -> F {
    let t70199 = t1041 * t248 * t3051 * t21134;
    let t70209 = t14508 * t17667;
    let t70211 = -t61695 / F::cast_from(288.0_f64) + t48431 - t3109 * t21597 / F::cast_from(576.0_f64) + t4641 * t17962 / F::cast_from(1024.0_f64) + t14085 * t5857 / F::cast_from(1536.0_f64) + t70199 / F::cast_from(6912.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3117 * t21580 + t42354 * t21405 / F::cast_from(3072.0_f64) + t61677 * t1622 / F::cast_from(1536.0_f64) - t50302 * t5875 / F::cast_from(96.0_f64) + t70209 / F::cast_from(768.0_f64);
    t70211
}
