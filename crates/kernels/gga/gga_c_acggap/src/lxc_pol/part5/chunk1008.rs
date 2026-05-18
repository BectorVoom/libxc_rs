//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1008/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1008<F: Float>(t1162: F, t16986: F, t4452: F, t5014: F, t997: F, t1352: F, t3700: F, t1181: F, t12936: F, t3655: F, t4643: F, t3044: F, t535: F) -> (F, F, F, F, F, F) {
    let t16987 = t16986 * t1162;
    let t16988 = t16987 * t4452;
    let t16990 = t997 * t5014;
    let t16992 = t3700 * t1352;
    let t16996 = t12936 * t1181 * t4643 * t3655;
    let t17000 = t12936 * t1181 * t535 * t3044;
    (t16987, t16988, t16990, t16992, t16996, t17000)
}
