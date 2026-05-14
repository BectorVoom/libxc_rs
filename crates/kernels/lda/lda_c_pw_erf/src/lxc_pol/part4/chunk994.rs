//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 994/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk994<F: Float>(t5401: F, t568: F, t10436: F, t548: F, t2114: F, t4564: F, t4568: F, t1529: F, t1960: F, t1466: F, t3667: F, t593: F, t833: F, t1401: F, t3899: F, t1318: F, t5271: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11936 = t5401 * t568;
    let t11940 = t548 * t10436;
    let t11946 = t2114 * t4564;
    let t11948 = t2114 * t4568;
    let t11954 = t1960 * t1529;
    let t11983 = t1466 * t3667;
    let t11984 = t833 * t593;
    let t11989 = t3899 * t1401;
    let t11991 = t1318 * t11989 * t5271;
    (t11936, t11940, t11946, t11948, t11954, t11983, t11984, t11989, t11991)
}
