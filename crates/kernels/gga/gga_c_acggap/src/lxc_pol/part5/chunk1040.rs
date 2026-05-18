//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1040/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1040<F: Float>(t1032: F, t5251: F, t12727: F, t1554: F, t1008: F, t5113: F, t13260: F, t1541: F, t3375: F, t4326: F, t1549: F, t16183: F, t2059: F) -> (F, F, F, F, F, F, F) {
    let t17930 = t1032 * t5251;
    let t17932 = t12727 * t1554;
    let t17938 = t1008 * t5113;
    let t17944 = t13260 * t1541;
    let t17946 = t3375 * t4326;
    let t17948 = t12727 * t1549;
    let t17951 = t16183 * t2059;
    (t17930, t17932, t17938, t17944, t17946, t17948, t17951)
}
