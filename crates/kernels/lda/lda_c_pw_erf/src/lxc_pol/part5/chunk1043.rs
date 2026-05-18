//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1043/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1043<F: Float>(t1234: F, t2443: F, t1386: F, t6597: F, t1590: F, t2379: F, t1124: F, t2363: F, t483: F, t485: F, t163: F, t169: F, t299: F, t7287: F) -> (F, F, F, F, F) {
    let t18710 = t2443 * t1234;
    let t18712 = t6597 * t1386;
    let t18735 = t2379 * t1590;
    let t18755 = t1124 * t2363 * t483 * t485;
    let t18761 = t169 * t299 * t7287 * t163;
    (t18710, t18712, t18735, t18755, t18761)
}
