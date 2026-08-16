//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 934/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk934<F: Float>(t1222: F, t1235: F, t1238: F, t1252: F, t1261: F, t1791: F, t3637: F, t3667: F, t3711: F, t5293: F, t5299: F, t5304: F, t5309: F, t5313: F, t5320: F, t5323: F, t5327: F, t5331: F, t5335: F) -> F {
    let t5338 = -F::cast_from(0.11433071498151929859e-2_f64) * t5293 * t1252 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t5299 + F::cast_from(0.23818898954483187207e-3_f64) * t1261 * t5304 - F::cast_from(0.95275595817932748827e-4_f64) * t3637 - t1222 * t5309 / F::cast_from(144.0_f64) + t1222 * t5313 / F::cast_from(216.0_f64) - F::cast_from(0.21437009059034868486e-3_f64) * t3667 * t1791 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t5320 + F::cast_from(0.11433071498151929859e-2_f64) * t5323 * t1238 - F::cast_from(0.21437009059034868486e-3_f64) * t5327 * t1238 - F::cast_from(0.21437009059034868486e-3_f64) * t5331 * t5335;
    t5338
}
