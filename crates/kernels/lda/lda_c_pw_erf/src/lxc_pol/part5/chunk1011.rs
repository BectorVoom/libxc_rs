//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1011/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1011<F: Float>(t1472: F, t7716: F, t16305: F, t743: F, t2017: F, t571: F, t34: F, t6365: F, t4868: F, t4753: F, t7720: F, t3416: F, t2411: F, t1318: F, t7724: F, t2065: F, t3832: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21206 = 4.0 / 9.0 * t1472 * t7716;
    let t21207 = t16305 * t743;
    let t21210 = 4.0 / 9.0 * t571 * t2017 * t21207;
    let t21211 = t6365 * t34;
    let t21214 = 8.0 / 9.0 * t571 * t4868 * t21211;
    let t21216 = 8.0 / 9.0 * t4753 * t7720;
    let t21218 = 8.0 / 9.0 * t3416 * t7720;
    let t21219 = t2411 * t34;
    let t21222 = 8.0 / 9.0 * t1318 * t4868 * t21219;
    let t21224 = 4.0 / 9.0 * t1472 * t7724;
    let t21228 = 4.0 / 9.0 * t571 * t3832 * t2411 * t2065;
    (t21206, t21207, t21210, t21211, t21214, t21216, t21218, t21219, t21222, t21224, t21228)
}
