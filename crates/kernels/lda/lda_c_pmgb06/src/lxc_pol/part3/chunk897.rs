//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 897/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk897<F: Float>(t12012: F, t1381: F, t493: F, t1382: F, t5305: F, t1972: F, t2980: F, t1420: F, t5483: F, t1423: F, t5345: F, t439: F, t4663: F, t5225: F, t2002: F, t2966: F) -> (F, F, F, F, F, F, F) {
    let t12015 = 2.0 / 15.0 * t493 * t12012 * t1381;
    let t12017 = 2.0 / 15.0 * t5305 * t1382;
    let t12019 = 2.0 / 15.0 * t1972 * t2980;
    let t12021 = 2.0 / 15.0 * t1420 * t5483;
    let t12022 = t1423 * t5345;
    let t12023 = 4.0 / 45.0 * t12022;
    let t12026 = 2.0 / 5.0 * t439 * t5225 * t4663;
    let t12028 = 2.0 / 15.0 * t2002 * t2966;
    (t12015, t12017, t12019, t12021, t12023, t12026, t12028)
}
