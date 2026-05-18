//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 683/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk683<F: Float>(t42: F, t987: F, t13: F, t25: F, t1448: F, t30: F, t14: F, t8: F, t82: F) -> (F, F, F, F, F, F) {
    let t4218 = t987 * t42;
    let t4494 = t13 * t13;
    let t4635 = t25 * t25;
    let t4772 = t30 * t1448;
    let t4793 = t14 * t13;
    let t4794 = F::new(1.0) / t4793;
    let t4803 = t8 * t82;
    (t4218, t4494, t4635, t4772, t4794, t4803)
}
