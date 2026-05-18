//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 982/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk982<F: Float>(t1030: F, t11356: F, t9262: F, t144: F, t8448: F, t1971: F, t9272: F, t1734: F, t5056: F, t1743: F, t5703: F, t3709: F) -> (F, F, F, F, F, F, F, F) {
    let t11357 = t1030 * t11356;
    let t11358 = t11357 * t9262;
    let t11360 = t8448 * t144;
    let t11361 = t1971 * t11360;
    let t11362 = t1030 * t11361;
    let t11363 = t11362 * t9272;
    let t11365 = t1734 * t5056;
    let t11367 = t1743 * t11365 * t5703;
    let t11369 = t3709 * t9262;
    (t11357, t11358, t11361, t11362, t11363, t11365, t11367, t11369)
}
