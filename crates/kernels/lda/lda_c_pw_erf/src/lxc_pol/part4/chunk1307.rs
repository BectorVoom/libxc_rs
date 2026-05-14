//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1307/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1307<F: Float>(t256: F, t652: F, t6880: F, t19: F, t6039: F, t644: F, t647: F, t1432: F, t2462: F, t1427: F, t6888: F, t11081: F, t11088: F, t17069: F, t17073: F, t17074: F, t17078: F, t17080: F, t17081: F, t17082: F, t17086: F, t17089: F, t17093: F, t17095: F, t247: F, t251: F) -> (F,) {
    let t19221 = t6880 * t652 * t256;
    let t19225 = t6039 * t19 * t644 * t647;
    let t19228 = t2462 * t1432 * t256;
    let t19230 = t6888 * t1427;
    let t19232 = 2.0 / 3.0 * t11081 + t11088 + t17095 * t247 * t251 * t256 / 3.0 + 2.0 / 3.0 * t19221 + 0.12155555555555556 * t19225 + t19228 / 3.0 + 0.12155555555555556 * t19230 + t17069 + t17073 - t17074 + t17078 - t17080 - t17081 + t17082 - t17086 + t17089 + t17093;
    (t19232,)
}
