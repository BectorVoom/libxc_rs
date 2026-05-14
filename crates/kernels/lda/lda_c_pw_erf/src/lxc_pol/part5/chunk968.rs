//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 968/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk968<F: Float>(t443: F, t7949: F, t1870: F, t5639: F, t7970: F, t14674: F, t1832: F, t1871: F, t20301: F, t20302: F, t20303: F, t2594: F, t2610: F, t411: F, t5651: F, t6121: F, t756: F, t7913: F, t7918: F, t8865: F, t8869: F, t8873: F, t9083: F) -> (F, F) {
    let t20440 = t7949 * t443;
    let t20493 = t1870 * t5639 * t7970;
    let t20507 = t8865 - t8869 + t8873 + 15.518295 * t1870 * t1871 * t756 * t6121 + 5.172765 * t1870 * t1871 * t7913 * t411 - 5.172765 * t20493 + 103.4553 * t1870 * t14674 * t7918 * t411 - 62.07318 * t1870 * t5651 * t2594 * t1832 + 15.518295 * t1870 * t1871 * t1832 * t2610 + t9083 - t20301 + t20302 - t20303;
    (t20440, t20507)
}
