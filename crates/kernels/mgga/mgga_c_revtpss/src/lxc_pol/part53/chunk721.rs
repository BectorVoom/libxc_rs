//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 721/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk721<F: Float>(t33: F, t265: F, t502: F, t1940: F, t30: F, t8490: F, t8494: F, t3140: F, t3268: F, t1078: F, t1035: F, t207: F, t8489: F, t8493: F, t198: F, t2411: F, t892: F, t57: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8498 = t1940 * t8490 * t30 / 2.0 - t1940 * t8494 * t30 / 2.0;
    let t8515 = t3140 * t3268;
    let t8520 = t3140 * t1078;
    let t8521 = t8520 * t1035;
    let t8536 = t207 * t8489;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    let t8552 = t1940 * t8490 * t33 / 2.0 - t1940 * t8494 * t33 / 2.0;
    let t8553 = piecewise3(t503, 0.0, t8542);
    let t8556 = piecewise3(t400, t8552, t8553 * t57 / 2.0);
    (t8498, t8515, t8521, t8536, t8539, t8542, t8553, t8556)
}
