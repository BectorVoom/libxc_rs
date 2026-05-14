//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 672/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk672<F: Float>(t2385: F, t4753: F, t3416: F, t1954: F, t743: F, t4758: F, t1318: F, t2000: F, t34: F, t2023: F, t2146: F, t2433: F, t542: F, t1313: F, t519: F, t2429: F, t348: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6475 = 16.0 / 45.0 * t4753 * t2385;
    let t6477 = 16.0 / 45.0 * t3416 * t2385;
    let t6478 = t1954 * t743;
    let t6479 = t4758 * t6478;
    let t6481 = 16.0 / 45.0 * t1318 * t6479;
    let t6482 = t2000 * t34;
    let t6483 = t4758 * t6482;
    let t6485 = 32.0 / 45.0 * t1318 * t6483;
    let t6487 = 8.0 / 45.0 * t2146 * t2023;
    let t6488 = t2433 * t542;
    let t6489 = t1313 * t6488;
    let t6491 = 8.0 / 45.0 * t519 * t6489;
    let t6492 = t2429 * t348;
    (t6475, t6477, t6478, t6479, t6481, t6482, t6483, t6485, t6487, t6488, t6489, t6491, t6492)
}
