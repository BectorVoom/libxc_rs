//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 808/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk808<F: Float>(t2754: F, t2759: F, t1861: F, t3532: F, t667: F, t204: F, t3515: F, t648: F) -> (F, F, F) {
    let t9140 = t2754 * t2759;
    let t9142 = t1861 * t3532;
    let t9143 = t9142 * t667;
    let t9148 = t204 * t648 * t3515;
    (t9140, t9143, t9148)
}
