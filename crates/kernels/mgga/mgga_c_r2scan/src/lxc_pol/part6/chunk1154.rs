//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1154/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1154<F: Float>(t189: F, t1906: F, t21066: F, t2: F, t273: F, t4: F, t3: F, t5243: F, t585: F, t409: F, t5308: F, t1266: F, t22: F, t1712: F, t2090: F, t5: F) -> (F, F, F, F, F, F, F, F) {
    let t21069 = 0.51288e1 * t1906 * t21066 * t189;
    let t21073 = t273 * t2 * t4;
    let t21074 = t585 * t5243 * t3 * t21073;
    let t21076 = t3 * t409;
    let t21077 = t5308 * t21076;
    let t21079 = t22 * t1266;
    let t21080 = t1712 * t21079;
    let t21082 = t5 * t2090;
    (t21069, t21073, t21074, t21076, t21077, t21079, t21080, t21082)
}
