//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1183/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1183<F: Float>(t161: F, t489: F, t4944: F, t1554: F, t2094: F, t486: F, t4948: F, t10431: F, t1385: F, t14068: F, t14115: F, t14136: F, t14160: F, t14198: F, t1444: F, t1512: F, t166: F, t2088: F, t2108: F, t2885: F, t2979: F, t3010: F, t3092: F, t3441: F, t439: F, t493: F, t4954: F, t518: F, t5276: F, t5277: F, t809: F, t822: F) -> F {
    let t14206 = t161 * t489 * t4944;
    let t14211 = t161 * t1554 * t2094;
    let t14212 = t14211 / F::cast_from(45.0_f64);
    let t14213 = t486 * t4948;
    let t14221 = -t439 * t1385 * t809 * t3441 / F::cast_from(45.0_f64) - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t439 * t10431 * t822 * t3092 * t3010 - t1444 * t5277 / F::cast_from(15.0_f64) - t493 * t2979 * t5276 / F::cast_from(15.0_f64) - t14068 / F::cast_from(15.0_f64) - t161 * t166 * t518 * (t14115 + t14136 + t14160 + t14198) / F::cast_from(30.0_f64) - t14206 / F::cast_from(15.0_f64) - t1512 * t2108 / F::cast_from(10.0_f64) + t14212 - F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t14213 - t161 * t166 * t2885 * t2088 / F::cast_from(10.0_f64) - t486 * t4954 / F::cast_from(10.0_f64);
    t14221
}
