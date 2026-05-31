//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1111/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1111<F: Float>(t30: F, t265: F, t393: F, t207: F, t7427: F, t1940: F, t198: F, t2071: F, t2403: F, t7432: F, t775: F, t890: F, t892: F, t2078: F, t45: F, t605: F, t606: F, t7010: F, t7092: F, t7428: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t7443 = t207 * t7427;
    let t7448 = -t1940 * t7432 * t890 + t198 * t7443 * t892 + F::cast_from(3.0_f64) * t2071 * t2403 * t775;
    let t7449 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t7448);
    let t7454 = piecewise3::<F>(t120, F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t7010 + t1940 * t7428 * t30 / F::cast_from(2.0_f64) - t1940 * t7432 * t7092 / F::cast_from(2.0_f64) + t1940 * t2071 * t605 / F::cast_from(2.0_f64), t2078 * t606 / F::cast_from(2.0_f64) + t7449 * t45 / F::cast_from(2.0_f64));
    (t7448, t7449, t7454)
}
