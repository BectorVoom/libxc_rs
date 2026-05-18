//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 794/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk794<F: Float>(t1086: F, t2777: F, t3371: F, t2811: F, t3396: F, t2979: F, t8117: F, t3388: F, t2520: F, t3392: F, t128: F, t147: F, t19: F) -> (F, F, F, F, F, F) {
    let t9485 = t1086 * t2777;
    let t9486 = t3371 * t9485;
    let t9488 = t3396 * t2811;
    let t9490 = t8117 * t2979;
    let t9491 = t9490 * t3388;
    let t9493 = t2520 * t2979;
    let t9494 = t9493 * t3392;
    let t9497 = t128 * t19 * t147;
    (t9485, t9486, t9488, t9491, t9494, t9497)
}
