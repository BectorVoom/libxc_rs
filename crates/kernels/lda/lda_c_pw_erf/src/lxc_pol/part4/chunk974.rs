//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 974/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk974<F: Float>(t10682: F, t3921: F, t1415: F, t1432: F, t256: F, t1427: F, t3946: F, t3949: F, t656: F, t1423: F, t3915: F, t3933: F, t1: F, t4166: F, t119: F, t1426: F, t3920: F) -> (F, F, F, F, F, F, F, F) {
    let t11038 = 0.008082336938271605 * t10682 * t3921;
    let t11053 = t1415 * t1432 * t256;
    let t11055 = t3946 * t1427;
    let t11057 = t3949 * t656;
    let t11060 = t1423 * t3915;
    let t11063 = 8.0 / 9.0 * t3933 * t656;
    let t11065 = t4166 * t1 * t3921;
    let t11069 = 0.006061752703703704 * t3920 * t119 * t1426;
    (t11038, t11053, t11055, t11057, t11060, t11063, t11065, t11069)
}
