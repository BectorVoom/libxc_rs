//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 929/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk929<F: Float>(t10075: F, t3207: F, t406: F, t2411: F, t3757: F, t824: F, t2888: F, t3026: F, t3175: F, t3730: F, t931: F, t6517: F, t919: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10102 = t10075 * t3207;
    let t10103 = t406 * t10102;
    let t10106 = t2411 * t3757;
    let t10107 = t10106 * t824;
    let t10108 = t2888 * t10107;
    let t10111 = t3175 * t3026;
    let t10112 = t2888 * t10111;
    let t10115 = t931 * t3730;
    let t10116 = t10115 * t824;
    let t10117 = t2888 * t10116;
    let t10121 = t6517 * t919;
    (t10102, t10103, t10106, t10107, t10108, t10111, t10112, t10115, t10116, t10117, t10121)
}
