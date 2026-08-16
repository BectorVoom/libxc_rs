//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1455/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1455<F: Float>(t11335: F, t11343: F, t2209: F, t2249: F, t5874: F, t1227: F, t2707: F, t38: F, t11320: F, t11322: F, t11330: F, t11341: F, t11354: F, t11357: F, t11364: F, t11407: F, t2229: F, t4394: F, t63: F, t6989: F) -> (F, F, F, F, F, F) {
    let t18644 = F::cast_from(5.84605_f64) * t11335;
    let t18646 = F::cast_from(1.9486833333333333_f64) * t11343;
    let t18649 = t2249 * t2209;
    let t18650 = t5874 * t18649;
    let t18656 = F::cast_from(5.84605_f64) * t38 * t2707 * t1227;
    let t18663 = F::cast_from(4.0_f64) * t11320 + F::cast_from(15.66976_f64) * t11322 - t11330 + t18644 - F::cast_from(5.87616_f64) * t11341 - t18646 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11354 - F::cast_from(11.75232_f64) * t11357 - F::cast_from(24.0_f64) * t11407 * t18650 + F::cast_from(29.3808_f64) * t11364 + t18656 - F::cast_from(29.3808_f64) * t63 * t6989 * t1227 + F::cast_from(11.75232_f64) * t63 * t2229 * t4394;
    (t18644, t18646, t18649, t18650, t18656, t18663)
}
