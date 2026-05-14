//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1322/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1322<F: Float>(t114: F, t2679: F, t5821: F, t856: F, t3032: F, t918: F, t15445: F, t9314: F, t2676: F, t43614: F, t2927: F, t31927: F, t31966: F, t111109: F, t111113: F, t111116: F, t111156: F, t111159: F, t111446: F) -> (F,) {
    let t111450 = t856 * t114 * t5821 * t2679;
    let t111454 = t856 * t3032 * t918 * t2679;
    let t111457 = t15445 * t9314 * t2679;
    let t111460 = t43614 * t2676 * t2679;
    let t111463 = t2927 * t31927 * t2679;
    let t111466 = t2927 * t31966 * t2679;
    let t111468 = -0.99491666666666666664e-2 * t111109 - 0.59694999999999999999e-1 * t111113 + 0.39796666666666666665e-1 * t111116 + 0.59694999999999999999e-1 * t111156 - 0.79593333333333333331e-1 * t111159 - 0.31250000000000000001e-1 * t111446 + 0.35108024691358024692e0 * t111450 + 0.72916666666666666668e-1 * t111454 + 0.72916666666666666668e-1 * t111457 - 0.10416666666666666667e-1 * t111460 + 0.14583333333333333334e0 * t111463 - 0.31250000000000000001e-1 * t111466;
    (t111468,)
}
