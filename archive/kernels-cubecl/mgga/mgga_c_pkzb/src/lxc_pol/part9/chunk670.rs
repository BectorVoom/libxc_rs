//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 670/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk670<F: Float>(t3073: F, t851: F, t2240: F, t2175: F, t2246: F, t3017: F, t3028: F, t1189: F, t862: F, t1197: F, t870: F, t2224: F, t2264: F, t2269: F, t3042: F, t3047: F, t3053: F, t3055: F, t3059: F, t3063: F, t3067: F) -> (F, F, F, F, F, F) {
    let t3074 = t3073 * t851;
    let t3076 = F::cast_from(0.16081979498692535067e2_f64) * t2240 * t3074;
    let t3080 = t2246 - F::cast_from(0.17123333333333333333e-1_f64) * t2175 - F::cast_from(0.17123333333333333333e-1_f64) * t3017 + F::cast_from(0.5137e-1_f64) * t3028;
    let t3083 = t1189 * t862;
    let t3088 = t1197 * t870;
    let t3102 = -F::cast_from(0.17648625e1_f64) * t3042 + F::cast_from(0.3529725e1_f64) * t3047 + t2264 - F::cast_from(0.516475e0_f64) * t2175 - F::cast_from(0.516475e0_f64) * t3017 + F::cast_from(0.1549425e1_f64) * t3028 + F::cast_from(0.31558125e0_f64) * t3053 + F::cast_from(0.6311625e0_f64) * t3055 + t2269 - F::cast_from(0.20839e0_f64) * t2224 - F::cast_from(0.20839e0_f64) * t3059 + F::cast_from(0.312585e0_f64) * t3063 + F::cast_from(0.312585e0_f64) * t3067;
    (t3074, t3076, t3080, t3083, t3088, t3102)
}
