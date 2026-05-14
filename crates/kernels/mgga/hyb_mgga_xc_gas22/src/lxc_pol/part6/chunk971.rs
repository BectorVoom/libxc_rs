//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 971/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk971<F: Float>(t9337: F, t9383: F, t9390: F, t9409: F, t541: F, t1175: F, t3656: F, t1528: F, t2944: F, t1563: F, t2817: F, t1115: F, t3792: F, t2893: F, t3771: F, t1539: F, t7692: F) -> (F, F, F, F, F, F, F, F) {
    let t9411 = t9337 + t9383 + t9390 + t9409;
    let t9412 = t9411 * t541;
    let t9413 = t3656 * t1175;
    let t9415 = t1528 * t2944;
    let t9416 = t2817 * t1563;
    let t9417 = t1115 * t3792;
    let t9419 = t3771 * t2893;
    let t9436 = t7692 * t1539;
    (t9411, t9412, t9413, t9415, t9416, t9417, t9419, t9436)
}
