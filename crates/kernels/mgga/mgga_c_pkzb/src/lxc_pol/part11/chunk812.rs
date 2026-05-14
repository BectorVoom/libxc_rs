//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 812/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk812<F: Float>(t1873: F, t3532: F, t667: F, t672: F, t9164: F, t218: F, t3542: F, t675: F) -> (F, F, F) {
    let t9177 = t1873 * t3532;
    let t9178 = t9177 * t667;
    let t9180 = t672 * t9164;
    let t9185 = t218 * t675 * t3542;
    (t9178, t9180, t9185)
}
