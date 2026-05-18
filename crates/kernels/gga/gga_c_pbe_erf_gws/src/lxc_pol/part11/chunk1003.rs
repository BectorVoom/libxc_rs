//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1003/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1003<F: Float>(t19733: F, t3912: F, t833: F, t3916: F, t4423: F, t19894: F, t3717: F, t945: F, t3928: F, t6854: F, t1033: F, t11025: F) -> (F, F, F, F, F, F) {
    let t39653 = t3912 * t19733 * t833;
    let t39661 = t3916 * t4423 * t833;
    let t39689 = t3912 * t19894;
    let t39749 = t945 * t3717;
    let t39758 = t3928 * t6854;
    let t39870 = t1033 * t11025;
    (t39653, t39661, t39689, t39749, t39758, t39870)
}
