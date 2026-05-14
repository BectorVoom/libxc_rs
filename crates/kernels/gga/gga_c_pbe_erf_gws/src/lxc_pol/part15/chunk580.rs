//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 580/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk580<F: Float>(t220: F, t34: F, t2735: F, t616: F, t1031: F, t202: F, t184: F, t619: F, t1019: F, t579: F, t1799: F, t1033: F, t636: F, t1045: F, t582: F, t211: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2736 = t220 * t34;
    let t2737 = t2735 * t2736;
    let t2739 = 4.0 / 15.0 * t616 * t2737;
    let t2740 = t202 * t1031;
    let t2741 = t2740 * t184;
    let t2743 = 4.0 / 15.0 * t2741 * t619;
    let t2745 = 2.0 / 15.0 * t579 * t1019;
    let t2746 = 8.0 / 45.0 * t1799;
    let t2747 = t1033 * t636;
    let t2748 = 4.0 / 45.0 * t2747;
    let t2749 = t582 * t1045;
    let t2750 = t211 * t2749;
    (t2736, t2737, t2739, t2740, t2741, t2743, t2745, t2746, t2748, t2749, t2750)
}
