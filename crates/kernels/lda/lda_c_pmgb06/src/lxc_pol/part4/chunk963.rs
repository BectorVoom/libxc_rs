//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 963/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk963<F: Float>(t1560: F, t5220: F, t443: F, t464: F, t1423: F, t5291: F, t1992: F, t3457: F, t517: F, t5312: F, t5345: F, t1710: F, t830: F, t500: F, t1417: F, t5194: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11952 = t5220 * t1560;
    let t11966 = t464 * t443;
    let t11971 = t1423 * t5291;
    let t12006 = t1992 * t3457;
    let t12012 = t5312 * t517;
    let t12022 = t1423 * t5345;
    let t12036 = t830 * t1710;
    let t12037 = t12036 * t500;
    let t12039 = t5194 * t1417;
    (t11952, t11966, t11971, t12006, t12012, t12022, t12036, t12037, t12039)
}
