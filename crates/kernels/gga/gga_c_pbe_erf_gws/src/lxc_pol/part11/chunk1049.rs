//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1049/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1049<F: Float>(t25594: F, t25608: F, t25619: F, t33963: F, t33965: F, t33967: F, t19229: F, t19232: F, t19249: F, t19316: F, t25773: F, t33973: F, t34045: F, t3644: F, t3637: F, t102: F, t1563: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t48725 = 0.77947333333333333333e1 * t25594;
    let t48727 = 0.60625703703703703703e1 * t25608;
    let t48728 = 0.51964888888888888888e1 * t25619;
    let t48729 = 0.38973666666666666666e1 * t33963;
    let t48730 = 0.77947333333333333333e1 * t33965;
    let t48731 = 0.38973666666666666666e1 * t33967;
    let t48733 = t48725 - 0.391744e1 * t25773 + t19229 - t19232 - t19249 + t19316 + t48727 - t48728 - t48729 + t48730 - t48731 + 0.2350464e2 * t33973;
    let t48736 = 0.19486833333333333333e1 * t34045;
    let t48737 = t3644 * t3644;
    let t48741 = t3637 * t3637;
    let t48747 = 0.701526e2 * t102 * t1563 * t48737;
    (t48725, t48727, t48728, t48729, t48730, t48731, t48733, t48736, t48737, t48741, t48747)
}
