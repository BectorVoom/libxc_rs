//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1193/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1193(t25594: f64, t25608: f64, t25619: f64, t33963: f64, t33965: f64, t33967: f64, t19229: f64, t19232: f64, t19249: f64, t19316: f64, t25773: f64, t33973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48725 = 0.77947333333333333333e1_f64 * t25594;
    let t48727 = 0.60625703703703703703e1_f64 * t25608;
    let t48728 = 0.51964888888888888888e1_f64 * t25619;
    let t48729 = 0.38973666666666666666e1_f64 * t33963;
    let t48730 = 0.77947333333333333333e1_f64 * t33965;
    let t48731 = 0.38973666666666666666e1_f64 * t33967;
    let t48733 = t48725 - 0.391744e1_f64 * t25773 + t19229 - t19232 - t19249 + t19316 + t48727 - t48728 - t48729 + t48730 - t48731 + 0.2350464e2_f64 * t33973;
    (t48725, t48727, t48728, t48729, t48730, t48731, t48733)
}
