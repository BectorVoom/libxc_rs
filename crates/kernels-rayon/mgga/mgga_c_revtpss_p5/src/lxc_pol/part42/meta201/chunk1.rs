//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 812/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk812(t1247: f64, t1252: f64, t1261: f64, t1797: f64, t3708: f64, t3711: f64, t484: f64, t5254: f64, t5256: f64, t5258: f64, t5262: f64, t5266: f64, t5270: f64, t5274: f64, t5279: f64, t5287: f64, t5338: f64, t5372: f64, t5410: f64) -> f64 {
    let t5412 = -0.7622047665434619906e-3_f64 * t5254 + 0.14291339372689912324e-3_f64 * t5256 - 0.11433071498151929859e-2_f64 * t5258 * t484 + 0.21437009059034868486e-3_f64 * t5262 * t484 + 0.14291339372689912324e-3_f64 * t5266 - 0.28582678745379824648e-3_f64 * t1261 * t5270 + 0.21437009059034868486e-3_f64 * t5274 * t1252 + 0.14291339372689912324e-3_f64 * t3711 * t5279 + 0.21437009059034868486e-3_f64 * t3708 * t1797 + 0.21437009059034868486e-3_f64 * t1247 * t5287 + t5338 + t5372 + t5410;
    t5412
}
