//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1286/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1286(t14469: f64, t53571: f64, t11819: f64, t51555: f64, t53236: f64, t14733: f64, t34838: f64, t353: f64, t859: f64, t14657: f64, t52993: f64, t13791: f64, t3916: f64) -> (f64, f64, f64, f64, f64) {
    let t56309 = t53571 * t14469;
    let t56312 = t51555 * t53236 * t11819;
    let t56316 = t14733 * t859 * t353 * t34838;
    let t56318 = t14657 * t52993;
    let t56320 = t3916 * t13791;
    (t56309, t56312, t56316, t56318, t56320)
}
