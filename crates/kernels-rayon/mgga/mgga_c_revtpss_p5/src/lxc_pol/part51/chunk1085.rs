//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1085/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1085(t7714: f64, t84: f64, t640: f64, t8621: f64, t33624: f64, t644: f64, t6972: f64, t1497: f64, t36: f64, t606: f64, t8442: f64, t119444: f64, t119456: f64, t119500: f64, t119503: f64, t119508: f64, t125280: f64, t125283: f64, t125286: f64, t125290: f64, t125294: f64, t125298: f64, t125305: f64, t125309: f64, t125313: f64, t125314: f64, t125319: f64, t32136: f64, t32142: f64, t32145: f64, t32149: f64, t33617: f64, t8438: f64, t8443: f64) -> f64 {
    let t125322 = t84 * t7714;
    let t125324 = t8621 * t125322 * t640;
    let t125328 = t8621 * t33624 * t644;
    let t125332 = t8621 * t33624 * t6972;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125337 = t8442 * t125336;
    let t125340 = -5.0_f64 / 18.0_f64 * t119456 * t125280 + 5.0_f64 / 144.0_f64 * t125283 * t8443 - 5.0_f64 / 36.0_f64 * t125286 * t32145 + 35.0_f64 / 24.0_f64 * t119503 * t125290 - 5.0_f64 / 12.0_f64 * t119508 * t125294 - 5.0_f64 / 12.0_f64 * t32136 * t125298 - 5.0_f64 / 36.0_f64 * t119500 * t33617 - 5.0_f64 / 36.0_f64 * t32142 * t125305 - 5.0_f64 / 36.0_f64 * t32142 * t125309 + 5.0_f64 / 18.0_f64 * t125313 * t8438 * t125314 - 5.0_f64 / 12.0_f64 * t119508 * t125319 + 5.0_f64 / 36.0_f64 * t32149 * t125324 - 5.0_f64 / 12.0_f64 * t32136 * t125328 + 5.0_f64 / 36.0_f64 * t32149 * t125332 + 5.0_f64 / 6.0_f64 * t119444 * t125337;
    t125340
}
