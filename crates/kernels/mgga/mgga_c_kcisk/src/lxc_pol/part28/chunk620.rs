//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 620/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk620<F: Float>(t1801: F, t7069: F, t1873: F, t1869: F, t1224: F, t2364: F, t4836: F) -> (F, F, F, F) {
    let t7070 = t1801 * t7069;
    let t7071 = t1873 * t7070;
    let t7072 = t1869 * t7071;
    let t7076 = t1224 * t4836 * t2364;
    (t7070, t7071, t7072, t7076)
}
