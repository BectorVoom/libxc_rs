//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 651/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk651<F: Float>(t1899: F, t5183: F, t1636: F, t5068: F, t5182: F, t1894: F, t4803: F, t1873: F, t1869: F, t1757: F) -> (F, F, F, F) {
    let t10426 = t5183 * t1899;
    let t10427 = t5068 * t1636;
    let t10428 = t10426 * t10427;
    let t10429 = t5182 * t10428;
    let t10431 = t4803 * t1894;
    let t10432 = t1899 * t10431;
    let t10433 = t1873 * t10432;
    let t10434 = t1869 * t10433;
    let t10436 = t4803 * t1757;
    (t10429, t10431, t10434, t10436)
}
