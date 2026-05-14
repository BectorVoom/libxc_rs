//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 632/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk632<F: Float>(t158: F, t3669: F, t1143: F, t2118: F, t2029: F, t3638: F) -> (F, F, F, F) {
    let t3670 = t3669 * t158;
    let t3675 = t1143 * t1143;
    let t3676 = t2118 * t3675;
    let t3679 = t3638 * t2029;
    (t3670, t3675, t3676, t3679)
}
