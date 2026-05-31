//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 350/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk350<F: Float>(t30: F, t265: F, t393: F, t1100: F, t1102: F, t198: F, t336: F, t895: F, t912: F, t938: F, t978: F, t980: F, t985: F, t395: F, t45: F, t605: F, t606: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t1106 = piecewise3::<F>(t394, t1100 * t1102 * t198 * t336 - t912 + t938 + t978 + t980 - t985, t895);
    let t1111 = piecewise3::<F>(t120, t265 * t605 / F::cast_from(2.0_f64) + t895 * t30 / F::cast_from(2.0_f64), t1106 * t45 / F::cast_from(2.0_f64) + t395 * t606 / F::cast_from(2.0_f64));
    (t1106, t1111)
}
