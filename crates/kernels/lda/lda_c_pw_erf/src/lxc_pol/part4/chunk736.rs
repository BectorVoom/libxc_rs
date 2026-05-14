//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 736/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk736<F: Float>(t1325: F, t4946: F, t1341: F, t2171: F, t1390: F, t1392: F, t784: F, t1440: F, t188: F, t473: F, t34: F, t529: F, t542: F, t2067: F, t565: F, t4915: F, t4917: F, t4919: F, t4921: F, t4923: F, t4925: F, t4927: F, t4932: F, t4935: F, t4940: F, t4945: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4948 = 16.0 / 45.0 * t1325 * t4946;
    let t4950 = 8.0 / 45.0 * t2171 * t1341;
    let t4952 = t1390 * t784 * t1392;
    let t4953 = t1440 * t4952;
    let t4955 = 8.0 / 15.0 * t1325 * t4953;
    let t4956 = t473 * t188;
    let t4957 = t529 * t34;
    let t4958 = t4957 * t542;
    let t4959 = t4956 * t4958;
    let t4961 = 8.0 / 15.0 * t1325 * t4959;
    let t4963 = 4.0 / 15.0 * t565 * t2067;
    let t4964 = -t4915 + t4917 + t4919 - t4921 - t4923 - t4925 - t4927 - t4932 - t4935 - t4940 + t4945 - t4948 - t4950 + t4955 - t4961 - t4963;
    (t4948, t4950, t4952, t4953, t4955, t4956, t4957, t4958, t4959, t4961, t4963, t4964)
}
