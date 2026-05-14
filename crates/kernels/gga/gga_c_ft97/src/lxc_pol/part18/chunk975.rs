//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 975/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk975<F: Float>(t26113: F, t488: F, t1307: F, t984: F, t1564: F, t379: F, t22873: F, t6421: F, t28: F, t497: F, t942: F, t5507: F, t6547: F, t11854: F, t1882: F, t6559: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26114 = t488 * t26113;
    let t26117 = t1307 * t984;
    let t26119 = t1564 * t26117 * t379;
    let t26124 = t22873 * t6421;
    let t26125 = t28 * t26124;
    let t26128 = t497 * t942;
    let t26129 = t5507 * t26128;
    let t26130 = t28 * t26129;
    let t26134 = t6547 * t379;
    let t26135 = t11854 * t26134;
    let t26139 = t1882 * t6559;
    (t26114, t26117, t26119, t26124, t26125, t26128, t26129, t26130, t26134, t26135, t26139)
}
