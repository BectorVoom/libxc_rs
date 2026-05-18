//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1010/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1010<F: Float>(t517: F, t5312: F, t1381: F, t493: F, t1382: F, t5305: F, t1972: F, t2980: F, t1420: F, t5483: F, t1423: F, t5345: F) -> (F, F, F, F, F) {
    let t12012 = t5312 * t517;
    let t12015 = F::new(2.0) / F::new(15.0) * t493 * t12012 * t1381;
    let t12017 = F::new(2.0) / F::new(15.0) * t5305 * t1382;
    let t12019 = F::new(2.0) / F::new(15.0) * t1972 * t2980;
    let t12021 = F::new(2.0) / F::new(15.0) * t1420 * t5483;
    let t12022 = t1423 * t5345;
    (t12015, t12017, t12019, t12021, t12022)
}
