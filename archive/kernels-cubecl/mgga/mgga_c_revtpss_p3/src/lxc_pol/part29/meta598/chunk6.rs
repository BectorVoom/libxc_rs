//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2033/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2033<F: Float>(t30: F, t265: F, t393: F, t103658: F, t103706: F, t102867: F, t102905: F, t103574: F, t103612: F, t13312: F, t1469: F, t2078: F, t2258: F, t26626: F, t28523: F, t4186: F, t45: F, t606: F, t7449: F, t8040: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t103707 = t103658 + t103706;
    let t103708 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t103707);
    let t103720 = piecewise3::<F>(t120, t102867 + t102905 + t103574 + t103612, t103708 * t45 / F::cast_from(2.0_f64) + t28523 * t606 + t8040 * t2258 / F::cast_from(2.0_f64) + t26626 * t1469 / F::cast_from(2.0_f64) + t7449 * t4186 + t2078 * t13312 / F::cast_from(2.0_f64));
    (t103707, t103720)
}
