//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3505/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3505<F: Float>(t1043: F, t4181: F, t1058: F, t19869: F, t3201: F, t6318: F, t1011: F, t15987: F, t18926: F, t18930: F, t16226: F, t19957: F, t19960: F, t19963: F, t3230: F, t3241: F, t375: F, t43174: F, t4915: F, t53320: F, t53328: F, t53832: F, t53859: F, t53875: F, t55209: F, t60927: F, t6317: F, t63313: F) -> (F, F) {
    let t66128 = t4181 * t1043;
    let t66139 = t19869 * t1058;
    let t66141 = t6318 * t3201;
    let t66155 = t1011 * t15987 * t18926;
    let t66158 = t1011 * t15987 * t18930;
    let t66161 = -F::cast_from(0.22866142996303859718e-2_f64) * t16226 * t55209 * t43174 * t66128 + t53320 * t53328 * t60927 / F::cast_from(12.0_f64) + F::cast_from(0.72409452821628889107e-2_f64) * t6317 * t3230 * t375 - F::cast_from(0.15244095330869239812e-2_f64) * t66139 - F::cast_from(0.47637797908966374413e-4_f64) * t66141 + F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t53832 - F::cast_from(0.28582678745379824648e-3_f64) * t53859 - F::cast_from(14.0_f64) / F::cast_from(243.0_f64) * t3241 * t19957 - t1011 * t4915 * t63313 / F::cast_from(12.0_f64) - t3241 * t19960 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t3241 * t19963 + t66155 / F::cast_from(72.0_f64) - t66158 / F::cast_from(108.0_f64) + F::cast_from(0.19055119163586549765e-3_f64) * t53875;
    (t66128, t66161)
}
