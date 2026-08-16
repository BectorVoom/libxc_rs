//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1024/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1024<F: Float>(t13298: F, t13299: F, t1432: F, t3196: F, t1165: F, t1532: F, t15758: F, t3451: F, t1539: F, t3194: F, t839: F, t1163: F, t1181: F, t4289: F, t5122: F) -> (F, F, F, F) {
    let t17430 = t13298 * t13299 * t1432 * t3196;
    let t17436 = t3451 * t1165 * t1532 * t15758;
    let t17441 = t3194 * t1165 * t1532 * t1539 * t839;
    let t17445 = t1163 * t1181 * t4289 * t5122;
    (t17430, t17436, t17441, t17445)
}
