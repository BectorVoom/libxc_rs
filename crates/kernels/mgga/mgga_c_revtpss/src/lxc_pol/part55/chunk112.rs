//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 112/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk112<F: Float>(t30: F, t33: F, t379: F, t385: F, t342: F, t198: F, t293: F, t328: F, t330: F, t336: F, t265: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t386 = t379 * t385;
    let t389 = 1.0 + 0.65854491829355115987e0 * t342 * t386;
    let t390 = f64::ln(t389);
    let t393 = t198 * t336 * t390 - t293 + t328 + t330;
    let t394 = t265 < t393;
    let t395 = piecewise3(t394, t393, t265);
    let t398 = piecewise3(t120, t265 * t30 / 2.0, t395 * t45 / 2.0);
    let t400 = rho1 <= dens_threshold || t34;
    let t403 = 1.0 / t57;
    let t404 = pow_1_3(t403);
    (t386, t389, t395, t398, t403, t404)
}
