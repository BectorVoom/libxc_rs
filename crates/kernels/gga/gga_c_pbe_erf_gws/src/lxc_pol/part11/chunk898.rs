//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 898/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk898<F: Float>(t3916: F, t6159: F, t2503: F, t8746: F, t3047: F, t3052: F, t26755: F, t3733: F, t19733: F, t3912: F, t833: F, t4423: F, t19894: F, t3717: F, t945: F, t3928: F, t6854: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39475 = t3916 * t6159;
    let t39490 = t8746 * t2503;
    let t39510 = t8746 * t3047;
    let t39521 = t8746 * t3052;
    let t39523 = t26755 * t3733;
    let t39653 = t3912 * t19733 * t833;
    let t39661 = t3916 * t4423 * t833;
    let t39689 = t3912 * t19894;
    let t39749 = t945 * t3717;
    let t39758 = t3928 * t6854;
    (t39475, t39490, t39510, t39521, t39523, t39653, t39661, t39689, t39749, t39758)
}
