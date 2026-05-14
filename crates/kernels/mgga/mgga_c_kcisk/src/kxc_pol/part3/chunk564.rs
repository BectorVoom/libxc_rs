//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 564/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk564<F: Float>(t1801: F, t5063: F, t5062: F, t1869: F, t1757: F, t1894: F, t1899: F, t1873: F, t140: F, t1797: F, t3737: F, t1803: F, t1802: F, t4581: F, t1799: F, t1333: F, t1865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5064 = t1801 * t5063;
    let t5065 = t5062 * t5064;
    let t5066 = t1869 * t5065;
    let t5068 = t1894 * t1757;
    let t5069 = t1899 * t5068;
    let t5070 = t1873 * t5069;
    let t5071 = t1869 * t5070;
    let t5074 = t140 * t3737 * t1797;
    let t5075 = t5074 * t1803;
    let t5077 = t4581 * t1802;
    let t5078 = t1799 * t5077;
    let t5080 = t1333 * t1865;
    (t5064, t5065, t5066, t5068, t5069, t5070, t5071, t5074, t5075, t5077, t5078, t5080)
}
