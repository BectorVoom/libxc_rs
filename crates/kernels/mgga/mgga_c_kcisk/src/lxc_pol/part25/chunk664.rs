//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 664/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk664<F: Float>(t1801: F, t7069: F, t1873: F, t1869: F, t1224: F, t2364: F, t4836: F, t4840: F, t6759: F, t1697: F, t6764: F, t673: F, t918: F, t6714: F, t4835: F, t4838: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7070 = t1801 * t7069;
    let t7071 = t1873 * t7070;
    let t7072 = t1869 * t7071;
    let t7076 = t1224 * t4836 * t2364;
    let t7079 = t1224 * t4840 * t6759;
    let t7082 = t1224 * t1697 * t6764;
    let t7084 = t918 * t673;
    let t7086 = t1224 * t7084 * t6714;
    let t7088 = t4835 + 0.5936111111111111111e-2 * t4838 + 0.5936111111111111111e-2 * t7076 - 0.11872222222222222222e-1 * t7079 + 0.35616666666666666666e-1 * t7082 + 0.35616666666666666666e-1 * t7086;
    (t7070, t7071, t7072, t7076, t7079, t7082, t7084, t7086, t7088)
}
