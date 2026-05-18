//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1102/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1102<F: Float>(t1455: F, t5305: F, t2002: F, t3263: F, t806: F, t9365: F, t1423: F, t4609: F, t5203: F, t439: F, t4608: F, t5197: F) -> (F, F, F, F, F, F) {
    let t13112 = t5305 * t1455 / F::new(15.0);
    let t13114 = F::new(8.0) / F::new(81.0) * t2002 * t3263;
    let t13116 = t9365 * t806 / F::new(45.0);
    let t13117 = t1423 * t4609;
    let t13118 = F::new(2.0) / F::new(15.0) * t13117;
    let t13119 = t1423 * t5203;
    let t13120 = F::new(4.0) / F::new(15.0) * t13119;
    let t13123 = t439 * t5197 * t4608 / F::new(5.0);
    (t13112, t13114, t13116, t13118, t13120, t13123)
}
