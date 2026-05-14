//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 735/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk735<F: Float>(t30: F, t33: F, t265: F, t393: F, t502: F, t1962: F, t207: F, t8656: F, t1940: F, t198: F, t7432: F, t892: F, t45: F, t8657: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8660 = t30 * t1962;
    let t8665 = t207 * t8656;
    let t8670 = -t1940 * t1962 * t7432 + t198 * t8665 * t892;
    let t8671 = piecewise3(t394, 0.0, t8670);
    let t8674 = piecewise3(t120, t1940 * t8657 * t30 / 2.0 - t1940 * t7432 * t8660 / 2.0, t8671 * t45 / 2.0);
    let t8677 = t33 * t1962;
    let t8682 = piecewise3(t503, 0.0, t8670);
    let t8685 = piecewise3(t400, t1940 * t8657 * t33 / 2.0 - t1940 * t7432 * t8677 / 2.0, t8682 * t57 / 2.0);
    let t8686 = t8674 + t8685;
    (t8660, t8665, t8671, t8677, t8682, t8686)
}
