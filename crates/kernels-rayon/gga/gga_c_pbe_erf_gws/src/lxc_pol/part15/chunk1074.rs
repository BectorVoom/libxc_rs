//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1074/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1074(t824: f64, t938: f64, t821: f64, t13781: f64, t3972: f64, t2190: f64, t3990: f64, t3991: f64, t3989: f64, t332: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13782 = t824 * t938;
    let t13783 = t821 * t13782;
    let t13784 = t13781 * t13783;
    let t13785 = t3972 * t13784;
    let t13788 = t3990 * t3991 * t2190;
    let t13789 = t3989 * t13788;
    let t13791 = t824 * t332;
    let t13792 = t822 * t13791;
    (t13782, t13783, t13784, t13785, t13788, t13789, t13791, t13792)
}
