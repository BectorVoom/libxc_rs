//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 814/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk814<F: Float>(t27261: F, t4368: F, t25223: F, t25229: F, t25235: F, t25243: F, t25254: F, t25276: F, t25278: F, t25284: F, t27244: F, t27246: F, t27249: F, t27251: F, t27254: F, t27256: F) -> (F, F) {
    let t27262 = t27261 * t4368;
    let t27264 = 7.0 / 144.0 * t25278 - t25284 - t27244 / 48.0 + 7.0 / 144.0 * t27246 - 0.10164000561857065645e-3 * t25235 + t25243 + t25276 - 0.17149607247227894789e-2 * t27249 - 0.10164000561857065645e-3 * t27251 + 0.14291339372689912324e-4 * t27254 + 0.80031500487063509015e-2 * t27256 + t25254 + 0.80031500487063509016e-2 * t25223 + 0.14291339372689912324e-4 * t25229 + 0.85748036236139473944e-3 * t27262;
    (t27262, t27264)
}
