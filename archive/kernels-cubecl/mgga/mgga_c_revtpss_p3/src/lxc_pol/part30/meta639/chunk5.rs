//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2221/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2221<F: Float>(t30: F, t265: F, t393: F, t100927: F, t13312: F, t1469: F, t2129: F, t2258: F, t26809: F, t28998: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, t99565: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t104438 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t100927);
    let t104450 = piecewise3::<F>(t120, t99565, t104438 * t45 / F::cast_from(2.0_f64) + t28998 * t606 + t8161 * t2258 / F::cast_from(2.0_f64) + t26809 * t1469 / F::cast_from(2.0_f64) + t7594 * t4186 + t2129 * t13312 / F::cast_from(2.0_f64));
    t104450
}
