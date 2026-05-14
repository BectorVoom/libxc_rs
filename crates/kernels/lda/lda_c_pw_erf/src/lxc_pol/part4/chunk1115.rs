//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1115/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1115<F: Float>(t325: F, t6643: F, t11: F, t1349: F, t15756: F, t6640: F, t2337: F, t3604: F, t951: F, t13562: F, t13564: F, t16274: F, t16276: F, t16278: F, t16280: F, t16285: F, t16287: F, t25: F, t589: F) -> (F, F, F, F, F, F) {
    let t16292 = t325 * t6643;
    let t16295 = t11 * t1349 * t15756;
    let t16297 = t325 * t6640;
    let t16299 = t3604 * t2337;
    let t16300 = t16299 * t951;
    let t16302 = t11 * t1349 * t16300;
    let t16304 = 0.03950617283950617 * t13562 + 0.2725925925925926 * t13564 + 0.8638 * t16276 - 0.21595 * t16280 + 0.16 * t25 * t589 * t16274 + 0.2311111111111111 * t16285 + 0.05333333333333334 * t16287 - 0.04 * t25 * t589 * t16278 - 0.09597777777777777 * t16292 - 0.8638 * t16295 + 0.026660493827160493 * t16297 + 0.14396666666666666 * t16302;
    (t16292, t16295, t16297, t16300, t16302, t16304)
}
