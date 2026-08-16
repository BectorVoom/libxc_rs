//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 912/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk912(t1248: f64, t471: f64, t5332: f64, t3720: f64, t1222: f64, t1235: f64, t1238: f64, t1252: f64, t1261: f64, t1791: f64, t3637: f64, t3667: f64, t3711: f64, t5293: f64, t5299: f64, t5304: f64, t5309: f64, t5313: f64, t5320: f64, t5323: f64, t5327: f64, t5331: f64) -> (f64, f64, f64, f64) {
    let t5333 = t1248 * t471;
    let t5334 = t5332 * t5333;
    let t5335 = t3720 * t5334;
    let t5338 = -0.11433071498151929859e-2_f64 * t5293 * t1252 + 0.14291339372689912324e-3_f64 * t3711 * t5299 + 0.23818898954483187207e-3_f64 * t1261 * t5304 - 0.95275595817932748827e-4_f64 * t3637 - t1222 * t5309 / 144.0_f64 + t1222 * t5313 / 216.0_f64 - 0.21437009059034868486e-3_f64 * t3667 * t1791 - 0.21437009059034868486e-3_f64 * t1235 * t5320 + 0.11433071498151929859e-2_f64 * t5323 * t1238 - 0.21437009059034868486e-3_f64 * t5327 * t1238 - 0.21437009059034868486e-3_f64 * t5331 * t5335;
    (t5333, t5334, t5335, t5338)
}
