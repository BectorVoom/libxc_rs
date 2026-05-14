//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1008/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1008<F: Float>(t12695: F, t1325: F, t5417: F, t10463: F, t2006: F, t1896: F, t603: F, t1513: F, t2134: F, t2127: F, t5069: F, t211: F, t5030: F, t514: F, t1508: F, t1440: F, t3675: F) -> (F, F, F, F, F, F, F, F) {
    let t12697 = t1325 * t12695 * t5417;
    let t12708 = t1325 * t10463 * t2006;
    let t12714 = t1896 * t603;
    let t12717 = t1513 * t2134;
    let t12723 = t5069 * t2127;
    let t12728 = t211 * t514 * t5030;
    let t12747 = t1508 * t2134;
    let t12765 = t1440 * t3675;
    (t12697, t12708, t12714, t12717, t12723, t12728, t12747, t12765)
}
