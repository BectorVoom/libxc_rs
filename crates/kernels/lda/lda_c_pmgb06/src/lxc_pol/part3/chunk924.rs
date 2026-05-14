//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 924/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk924<F: Float>(t2945: F, t831: F, t1548: F, t1887: F, t2857: F, t802: F, t3134: F, t815: F, t1512: F, t1874: F, t161: F, t3004: F, t852: F, t432: F, t4817: F, t835: F, t9266: F) -> (F, F, F, F, F, F, F, F) {
    let t12446 = t831 * t2945 / 30.0;
    let t12447 = t1887 * t1548;
    let t12448 = t12447 / 45.0;
    let t12449 = t802 * t2857;
    let t12450 = t12449 / 45.0;
    let t12452 = t3134 * t815 / 30.0;
    let t12454 = t1512 * t1874 / 10.0;
    let t12456 = t161 * t3004 * t852;
    let t12457 = 4.0 / 405.0 * t12456;
    let t12459 = t432 * t4817 / 5.0;
    let t12460 = t9266 * t835;
    (t12446, t12448, t12450, t12452, t12454, t12457, t12459, t12460)
}
