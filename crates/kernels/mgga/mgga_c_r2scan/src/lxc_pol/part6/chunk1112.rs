//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1112/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1112<F: Float>(t2133: F, t2135: F, t6848: F, t2139: F, t2141: F, t2294: F, t6253: F, t6170: F, t1567: F, t5073: F, t6359: F, t259: F, t6180: F, t546: F, t565: F, t6118: F, t6283: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19929 = t2133 * t6848 * t2135;
    let t19932 = t2139 * t6848 * t2141;
    let t19948 = t2133 * t2294 * t6253;
    let t19951 = t2139 * t2294 * t6170;
    let t19965 = t1567 * t5073;
    let t19977 = t6359 * t5073;
    let t19986 = t6180 * t259;
    let t19987 = t546 * t19986;
    let t19990 = t565 * t19986;
    let t19999 = t6118 * t6283;
    (t19929, t19932, t19948, t19951, t19965, t19977, t19986, t19987, t19990, t19999)
}
