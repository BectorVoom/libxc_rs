//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 97/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk97<F: Float>(t281: F, t282: F, t283: F, t273: F, t276: F, t279: F, t275: F, t153: F, t159: F, t162: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t285 = t281 * t282 * t283;
    let t287 = F::new(0.379785e1) * t276 + F::new(0.8969e0) * t273 + F::new(0.204775e0) * t279 + F::new(0.123235e0) * t285;
    let t290 = F::new(1.0) + F::new(0.16081979498692535067e2) / t287;
    let t291 = f64::ln(t290);
    let t293 = F::new(0.621814e-1) * t275 * t291;
    let t294 = F::new(2.0) <= zeta_threshold;
    let t296 = piecewise3::<f64>(t294, t153, F::new(2.0) * t159);
    let t297 = F::new(0.0) <= zeta_threshold;
    let t298 = piecewise3::<f64>(t297, t153, F::new(0.0));
    let t300 = (t296 + t298 - F::new(2.0)) * t162;
    (t285, t287, t290, t291, t293, t300)
}
