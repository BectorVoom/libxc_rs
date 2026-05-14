//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 959/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk959<F: Float>(t125335: F, t606: F, t8442: F, t119444: F, t119456: F, t119500: F, t119503: F, t119508: F, t125280: F, t125283: F, t125286: F, t125290: F, t125294: F, t125298: F, t125305: F, t125309: F, t125313: F, t125314: F, t125319: F, t125324: F, t125328: F, t125332: F, t32136: F, t32142: F, t32145: F, t32149: F, t33617: F, t8438: F, t8443: F) -> (F,) {
    let t125336 = t125335 * t606;
    let t125337 = t8442 * t125336;
    let t125340 = -5.0 / 18.0 * t119456 * t125280 + 5.0 / 144.0 * t125283 * t8443 - 5.0 / 36.0 * t125286 * t32145 + 35.0 / 24.0 * t119503 * t125290 - 5.0 / 12.0 * t119508 * t125294 - 5.0 / 12.0 * t32136 * t125298 - 5.0 / 36.0 * t119500 * t33617 - 5.0 / 36.0 * t32142 * t125305 - 5.0 / 36.0 * t32142 * t125309 + 5.0 / 18.0 * t125313 * t8438 * t125314 - 5.0 / 12.0 * t119508 * t125319 + 5.0 / 36.0 * t32149 * t125324 - 5.0 / 12.0 * t32136 * t125328 + 5.0 / 36.0 * t32149 * t125332 + 5.0 / 6.0 * t119444 * t125337;
    (t125340,)
}
