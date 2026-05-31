//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 746/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk746<F: Float>(t30: F, t265: F, t393: F, t207: F, t8493: F, t198: F, t2411: F, t8536: F, t892: F, t1102: F, t3336: F, t336: F, t8527: F, t8531: F, t45: F, t8498: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    let t8543 = piecewise3::<F>(t394, t1102 * t198 * t336 * t8527 - t198 * t3336 * t336 * t8531, t8542);
    let t8546 = piecewise3::<F>(t120, t8498, t8543 * t45 / F::cast_from(2.0_f64));
    (t8539, t8542, t8543, t8546)
}
