//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 959/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk959<F: Float>(t1634: F, t695: F, t2070: F, t493: F, t495: F, t1393: F, t1518: F, t185: F, t4039: F, t511: F, t220: F, t4567: F, t211: F, t1524: F, t1529: F, t197: F, t3783: F) -> (F, F, F, F, F, F, F, F) {
    let t10417 = 0.004413481481481482 * t695 * t1634;
    let t10419 = t493 * t2070 * t495;
    let t10422 = t185 * t1518 * t1393;
    let t10427 = t511 * t4039;
    let t10436 = t4567 * t220;
    let t10438 = 112.0 / 1215.0 * t211 * t10436;
    let t10439 = t1524 * t1529;
    let t10463 = t3783 * t197;
    (t10417, t10419, t10422, t10427, t10436, t10438, t10439, t10463)
}
