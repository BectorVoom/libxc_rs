//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1075/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1075<F: Float>(t1287: F, t1318: F, t1319: F, t2419: F, t4738: F, t5276: F, t5266: F, t1325: F, t1326: F, t6263: F, t945: F, t1245: F, t1991: F, t2328: F, t940: F, t2146: F, t4795: F) -> (F, F, F, F, F, F) {
    let t15658 = 8.0 / 45.0 * t1318 * t1319 * t2419 * t1287;
    let t15660 = 16.0 / 45.0 * t4738 * t5276;
    let t15662 = 16.0 / 27.0 * t4738 * t5266;
    let t15666 = 8.0 / 45.0 * t1325 * t1326 * t6263 * t945;
    let t15671 = 8.0 / 27.0 * t1325 * t1991 * t2328 * t1245 * t940;
    let t15672 = t2146 * t4795;
    (t15658, t15660, t15662, t15666, t15671, t15672)
}
