//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1242/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1242<F: Float>(t3416: F, t6244: F, t1318: F, t2065: F, t5269: F, t593: F, t811: F, t1381: F, t6242: F, t1401: F, t17865: F, t4892: F, t12874: F, t2035: F, t4763: F, t4781: F) -> (F, F, F, F, F, F) {
    let t18454 = 32.0 / 15.0 * t3416 * t6244;
    let t18459 = 32.0 / 15.0 * t1318 * t5269 * t811 * t2065 * t593;
    let t18463 = 16.0 / 15.0 * t1318 * t5269 * t6242 * t1381;
    let t18467 = 32.0 / 15.0 * t1318 * t4892 * t1401 * t17865;
    let t18469 = 32.0 / 45.0 * t12874 * t2035;
    let t18471 = 16.0 / 45.0 * t4763 * t4781;
    (t18454, t18459, t18463, t18467, t18469, t18471)
}
