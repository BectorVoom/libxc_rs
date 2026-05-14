//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1184/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1184<F: Float>(t12874: F, t2193: F, t4763: F, t4895: F, t1287: F, t1318: F, t1466: F, t6963: F, t3899: F, t571: F, t6969: F, t6973: F, t1325: F, t16974: F, t197: F, t35: F, t504: F) -> (F, F, F, F, F, F) {
    let t17497 = 16.0 / 15.0 * t12874 * t2193;
    let t17499 = 16.0 / 15.0 * t4763 * t4895;
    let t17503 = 8.0 / 15.0 * t1318 * t1466 * t6963 * t1287;
    let t17505 = t571 * t3899 * t6969;
    let t17506 = 16.0 / 15.0 * t17505;
    let t17508 = t571 * t3899 * t6973;
    let t17509 = 32.0 / 45.0 * t17508;
    let t17514 = 64.0 / 45.0 * t1325 * t16974 * t197 * t35 * t504;
    (t17497, t17499, t17503, t17506, t17509, t17514)
}
