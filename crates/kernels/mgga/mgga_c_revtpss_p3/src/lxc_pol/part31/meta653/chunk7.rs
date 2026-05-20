//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2183/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2183<F: Float>(t33: F, t265: F, t502: F, t107922: F, t107963: F, t108001: F, t108047: F, t107868: F, t1469: F, t18281: F, t2003: F, t27822: F, t29978: F, t4186: F, t57: F, t5825: F, t606: F, t7215: F, t7877: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t108049 = t107922 + t107963 + t108001 + t108047;
    let t108050 = piecewise3::<F>(t503, F::new(0.0), t107868);
    let t108062 = piecewise3::<F>(t400, t108049, t108050 * t57 / F::new(2.0) - t29978 * t606 / F::new(2.0) - t27822 * t1469 - t7877 * t4186 - t7215 * t5825 / F::new(2.0) - t2003 * t18281 / F::new(2.0));
    t108062
}
