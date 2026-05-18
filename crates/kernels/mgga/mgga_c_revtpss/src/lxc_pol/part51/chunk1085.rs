//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1085/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1085<F: Float>(t7714: F, t84: F, t640: F, t8621: F, t33624: F, t644: F, t6972: F, t1497: F, t36: F, t606: F, t8442: F, t119444: F, t119456: F, t119500: F, t119503: F, t119508: F, t125280: F, t125283: F, t125286: F, t125290: F, t125294: F, t125298: F, t125305: F, t125309: F, t125313: F, t125314: F, t125319: F, t32136: F, t32142: F, t32145: F, t32149: F, t33617: F, t8438: F, t8443: F) -> F {
    let t125322 = t84 * t7714;
    let t125324 = t8621 * t125322 * t640;
    let t125328 = t8621 * t33624 * t644;
    let t125332 = t8621 * t33624 * t6972;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125337 = t8442 * t125336;
    let t125340 = -F::new(5.0) / F::new(18.0) * t119456 * t125280 + F::new(5.0) / F::new(144.0) * t125283 * t8443 - F::new(5.0) / F::new(36.0) * t125286 * t32145 + F::new(35.0) / F::new(24.0) * t119503 * t125290 - F::new(5.0) / F::new(12.0) * t119508 * t125294 - F::new(5.0) / F::new(12.0) * t32136 * t125298 - F::new(5.0) / F::new(36.0) * t119500 * t33617 - F::new(5.0) / F::new(36.0) * t32142 * t125305 - F::new(5.0) / F::new(36.0) * t32142 * t125309 + F::new(5.0) / F::new(18.0) * t125313 * t8438 * t125314 - F::new(5.0) / F::new(12.0) * t119508 * t125319 + F::new(5.0) / F::new(36.0) * t32149 * t125324 - F::new(5.0) / F::new(12.0) * t32136 * t125328 + F::new(5.0) / F::new(36.0) * t32149 * t125332 + F::new(5.0) / F::new(6.0) * t119444 * t125337;
    t125340
}
