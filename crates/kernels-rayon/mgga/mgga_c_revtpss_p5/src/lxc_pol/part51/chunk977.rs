//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 977/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk977(t32366: f64, t572: f64, t7002: f64, t7330: f64, t1459: f64, t8614: f64, t116: f64, t8460: f64, t670: f64, t1461: f64, t32354: f64, t32358: f64, t32360: f64, t32362: f64, t32365: f64, t573: f64, t8607: f64, t8616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32368 = 6.0_f64 * t572 * t32366;
    let t32369 = t7330 * t7002;
    let t32371 = 12.0_f64 * t572 * t32369;
    let t32372 = t1459 * t8614;
    let t32373 = 3.0_f64 * t32372;
    let t32374 = t116 * t8460;
    let t32375 = t32374 * t670;
    let t32376 = t572 * t32375;
    let t32377 = 6.0_f64 * t32376;
    let t32378 = 3.0_f64 * t1461 * t8607 + t32354 * t573 + 6.0_f64 * t32358 + 12.0_f64 * t32360 + 6.0_f64 * t32362 + t32365 + t32368 + t32371 + t32373 + t32377 + t8616;
    (t32369, t32373, t32374, t32375, t32377, t32378)
}
