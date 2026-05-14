//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 844/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk844<F: Float>(t30: F, t265: F, t393: F, t31830: F, t31837: F, t7002: F, t93: F, t1419: F, t3140: F, t8477: F, t25875: F, t32275: F, t32268: F, t32237: F, t32058: F, t31882: F, t45: F, t606: F, t8752: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t32474 = t31830 * t31837;
    let t32655 = t93 * t7002;
    let t32699 = t1419 * t3140;
    let t32700 = t8477 * t32699;
    let t32705 = t25875 * t32275;
    let t32710 = t32268 * t32275;
    let t32719 = t8477 * t32237;
    let t32785 = piecewise3(t394, 0.0, t32058);
    let t32790 = piecewise3(t120, t31882, t32785 * t45 / 2.0 + t8752 * t606 / 2.0);
    (t32474, t32655, t32699, t32700, t32705, t32710, t32719, t32785, t32790)
}
