//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 948/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk948<F: Float>(t30: F, t265: F, t393: F, t32268: F, t32275: F, t32237: F, t8477: F, t32058: F, t31882: F, t45: F, t606: F, t8752: F, t10301: F, t8736: F, t10309: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t32710 = t32268 * t32275;
    let t32719 = t8477 * t32237;
    let t32785 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t32058);
    let t32790 = piecewise3::<F>(t120, t31882, t32785 * t45 / F::cast_from(2.0_f64) + t8752 * t606 / F::cast_from(2.0_f64));
    let t32795 = t10301 * t8736;
    let t32798 = t10309 * t8736;
    (t32710, t32719, t32785, t32790, t32795, t32798)
}
