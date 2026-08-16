//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1178/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1178<F: Float>(t11938: F, t325: F, t9386: F, t1044: F, t654: F, t11803: F, t11804: F, t19196: F, t11775: F, t29207: F, t147: F, t19: F, t512: F) -> (F, F, F, F, F, F) {
    let t33710 = t325 * t9386 * t11938;
    let t33712 = t654 * t1044;
    let t33714 = t325 * t33712 * t11938;
    let t33717 = t11803 * t11804 * t19196;
    let t33719 = t11775 * t29207;
    let t33722 = t512 * t19 * t147;
    (t33710, t33712, t33714, t33717, t33719, t33722)
}
