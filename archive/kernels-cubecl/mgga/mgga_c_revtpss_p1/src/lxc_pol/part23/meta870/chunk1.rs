//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2769/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2769<F: Float>(t22298: F, t48100: F, t9816: F, t22129: F, t2713: F, t3964: F, t22169: F, t46691: F, t22173: F, t9744: F, t6856: F, t9779: F) -> (F, F, F, F, F) {
    let t74257 = t9816 * t48100 * t22298;
    let t74264 = t3964 * t2713 * t22129;
    let t74269 = t46691 * t22169;
    let t74271 = t9744 * t22173;
    let t74277 = t9779 * t6856;
    (t74257, t74264, t74269, t74271, t74277)
}
