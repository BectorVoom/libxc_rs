//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1193/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1193<F: Float>(t25594: F, t25608: F, t25619: F, t33963: F, t33965: F, t33967: F, t19229: F, t19232: F, t19249: F, t19316: F, t25773: F, t33973: F) -> (F, F, F, F, F, F, F) {
    let t48725 = F::new(0.77947333333333333333e1) * t25594;
    let t48727 = F::new(0.60625703703703703703e1) * t25608;
    let t48728 = F::new(0.51964888888888888888e1) * t25619;
    let t48729 = F::new(0.38973666666666666666e1) * t33963;
    let t48730 = F::new(0.77947333333333333333e1) * t33965;
    let t48731 = F::new(0.38973666666666666666e1) * t33967;
    let t48733 = t48725 - F::new(0.391744e1) * t25773 + t19229 - t19232 - t19249 + t19316 + t48727 - t48728 - t48729 + t48730 - t48731 + F::new(0.2350464e2) * t33973;
    (t48725, t48727, t48728, t48729, t48730, t48731, t48733)
}
