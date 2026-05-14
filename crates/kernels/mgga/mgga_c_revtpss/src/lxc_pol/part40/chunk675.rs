//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 675/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk675<F: Float>(t30: F, t265: F, t393: F, t2838: F, t3339: F, t1106: F, t2257: F, t2258: F, t395: F, t45: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t3340 = piecewise3(t394, t3339, t2838);
    let t3347 = piecewise3(t120, t2838 * t30 / 2.0 + t895 * t605 + t265 * t2257 / 2.0, t3340 * t45 / 2.0 + t1106 * t606 + t395 * t2258 / 2.0);
    let t3351 = -t2257;
    (t3340, t3347, t3351)
}
