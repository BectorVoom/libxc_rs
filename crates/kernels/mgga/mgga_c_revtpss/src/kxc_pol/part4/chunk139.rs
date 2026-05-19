//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 139/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk139<F: Float>(t30: F, t33: F, t265: F, t395: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t398 = piecewise3::<F>(t120, t265 * t30 / F::new(2.0), t395 * t45 / F::new(2.0));
    let t400 = rho1 <= dens_threshold || t34;
    let t403 = F::new(1.0) / t57;
    let t404 = pow_1_3::<F>(t403);
    (t398, t403, t404)
}
