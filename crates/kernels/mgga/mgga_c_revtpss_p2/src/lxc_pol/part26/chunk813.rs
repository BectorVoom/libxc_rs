//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 813/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk813<F: Float>(t45: F, t57: F, t2375: F, t606: F, t10326: F, t10356: F, t10446: F, t2258: F, t78: F, t202: F, t2382: F, t81: F, t150: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10449 = t2375 * t606;
    let t10455 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10446 * t10356 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10449 * t2258 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t10326);
    let t10457 = F::cast_from(1.0_f64) / t202 / t57;
    let t10460 = t2382 * t606;
    let t10466 = piecewise3::<F>(t155, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10457 * t10356 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10460 * t2258 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81 * t10326);
    let t10467 = t10455 + t10466;
    let t10468 = t150 * t10467;
    (t10467, t10468)
}
