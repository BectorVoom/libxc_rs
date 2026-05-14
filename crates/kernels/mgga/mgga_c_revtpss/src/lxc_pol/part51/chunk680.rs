//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 680/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk680<F: Float>(t30: F, t33: F, t265: F, t393: F, t502: F, t1102: F, t198: F, t3336: F, t336: F, t8527: F, t8531: F, t8542: F, t45: F, t8498: F, t1940: F, t8490: F, t8494: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8543 = piecewise3(t394, t1102 * t198 * t336 * t8527 - t198 * t3336 * t336 * t8531, t8542);
    let t8546 = piecewise3(t120, t8498, t8543 * t45 / 2.0);
    let t8552 = t1940 * t8490 * t33 / 2.0 - t1940 * t8494 * t33 / 2.0;
    let t8553 = piecewise3(t503, 0.0, t8542);
    let t8556 = piecewise3(t400, t8552, t8553 * t57 / 2.0);
    let t8557 = t8546 + t8556;
    (t8543, t8553, t8557)
}
