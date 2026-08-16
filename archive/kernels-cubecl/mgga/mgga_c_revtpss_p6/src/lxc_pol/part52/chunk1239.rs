//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1239/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1239<F: Float>(t28042: F, t7359: F, t108120: F, t122570: F, t125362: F, t125365: F, t127535: F, t128291: F, t128331: F, t128332: F, t128333: F, t128335: F, t128337: F, t128338: F, t128339: F, t128340: F, t1518: F, t2055: F, t25805: F, t28030: F, t32389: F, t33602: F, t4292: F, t670: F, t7373: F, t7983: F, t97622: F) -> F {
    let t128341 = t7359 * t28042;
    let t128349 = t108120 * t2055 + t122570 * t1518 + t125362 * t2055 + t125365 * t2055 + t127535 * t1518 + t128291 * t670 + t2055 * t97622 + t25805 * t7983 + t28030 * t7373 + t32389 * t4292 + t33602 * t7373 + t128331 + t128332 + t128333 + t128335 + t128337 + t128338 + t128339 + t128340 + t128341;
    t128349
}
