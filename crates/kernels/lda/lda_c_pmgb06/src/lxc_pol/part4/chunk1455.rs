//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1455/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1455<F: Float>(t11335: F, t11343: F, t2209: F, t2249: F, t5874: F, t1227: F, t2707: F, t38: F, t11320: F, t11322: F, t11330: F, t11341: F, t11354: F, t11357: F, t11364: F, t11407: F, t2229: F, t4394: F, t63: F, t6989: F) -> (F, F, F, F, F, F) {
    let t18644 = F::new(5.84605) * t11335;
    let t18646 = F::new(1.9486833333333333) * t11343;
    let t18649 = t2249 * t2209;
    let t18650 = t5874 * t18649;
    let t18656 = F::new(5.84605) * t38 * t2707 * t1227;
    let t18663 = F::new(4.0) * t11320 + F::new(15.66976) * t11322 - t11330 + t18644 - F::new(5.87616) * t11341 - t18646 + F::new(8.0) / F::new(3.0) * t11354 - F::new(11.75232) * t11357 - F::new(24.0) * t11407 * t18650 + F::new(29.3808) * t11364 + t18656 - F::new(29.3808) * t63 * t6989 * t1227 + F::new(11.75232) * t63 * t2229 * t4394;
    (t18644, t18646, t18649, t18650, t18656, t18663)
}
