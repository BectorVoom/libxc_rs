//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 817/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk817<F: Float>(t1554: F, t1603: F, t161: F, t3457: F, t496: F, t1382: F, t3223: F, t1447: F, t2988: F, t3194: F, t517: F, t3383: F, t489: F, t2060: F, t526: F, t3344: F, t405: F) -> (F, F, F, F, F, F, F, F) {
    let t9898 = t161 * t1554 * t1603;
    let t9908 = t496 * t3457;
    let t9921 = t3223 * t1382;
    let t9923 = t1447 * t2988;
    let t9925 = t3194 * t517;
    let t9936 = t161 * t489 * t3383;
    let t9938 = t2060 * t526;
    let t9940 = t405 * t3344;
    (t9898, t9908, t9921, t9923, t9925, t9936, t9938, t9940)
}
