//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 783/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk783<F: Float>(t1247: F, t1252: F, t1261: F, t1797: F, t3708: F, t3711: F, t484: F, t5254: F, t5256: F, t5258: F, t5262: F, t5266: F, t5270: F, t5274: F, t5279: F, t5287: F, t5338: F, t5372: F, t5410: F) -> (F,) {
    let t5412 = -0.7622047665434619906e-3 * t5254 + 0.14291339372689912324e-3 * t5256 - 0.11433071498151929859e-2 * t5258 * t484 + 0.21437009059034868486e-3 * t5262 * t484 + 0.14291339372689912324e-3 * t5266 - 0.28582678745379824648e-3 * t1261 * t5270 + 0.21437009059034868486e-3 * t5274 * t1252 + 0.14291339372689912324e-3 * t3711 * t5279 + 0.21437009059034868486e-3 * t3708 * t1797 + 0.21437009059034868486e-3 * t1247 * t5287 + t5338 + t5372 + t5410;
    (t5412,)
}
