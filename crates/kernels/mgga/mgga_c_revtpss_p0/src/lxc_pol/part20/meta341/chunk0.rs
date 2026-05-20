//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1267/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1267<F: Float>(t1012: F, t11821: F, t3133: F, t357: F, t3059: F, t3075: F, t11670: F, t4890: F, t3317: F, t3299: F, t12047: F, t15905: F) -> (F, F, F, F, F, F, F, F) {
    let t16012 = t1012 * t11821;
    let t16020 = t3133 * t357;
    let t16025 = t357 * t3059;
    let t16043 = t357 * t3075;
    let t16048 = t11670 * t4890;
    let t16049 = t3317 * t16048;
    let t16052 = t3299 * t16048;
    let t16067 = t12047 * t15905;
    (t16012, t16020, t16025, t16043, t16048, t16049, t16052, t16067)
}
