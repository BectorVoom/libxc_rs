//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 366/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk366<F: Float>(t33: F, t1113: F, t1304: F, t265: F, t504: F, t57: F, t606: F, t895: F, t1111: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t1309 = piecewise3::<F>(t400, t265 * t1113 / F::cast_from(2.0_f64) + t895 * t33 / F::cast_from(2.0_f64), t1304 * t57 / F::cast_from(2.0_f64) - t504 * t606 / F::cast_from(2.0_f64));
    let t1310 = t1111 + t1309;
    t1310
}
