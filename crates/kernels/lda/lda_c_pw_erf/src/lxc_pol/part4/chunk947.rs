//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 947/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk947<F: Float>(t1318: F, t1468: F, t9432: F, t2151: F, t576: F, t571: F, t1390: F, t1459: F, t2070: F, t548: F, t550: F, t1404: F, t1518: F, t211: F, t1472: F, t3763: F) -> (F, F, F, F, F, F) {
    let t9434 = t1318 * t9432 * t1468;
    let t9436 = t2151 * t576;
    let t9437 = t571 * t9436;
    let t9504 = t1459 * t1390;
    let t9593 = t548 * t2070 * t550;
    let t9596 = t211 * t1518 * t1404;
    let t9645 = t1472 * t3763;
    (t9434, t9437, t9504, t9593, t9596, t9645)
}
