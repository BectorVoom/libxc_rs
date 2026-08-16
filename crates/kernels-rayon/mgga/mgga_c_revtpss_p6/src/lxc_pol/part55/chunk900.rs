//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 900/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk900(t241: f64, t25260: f64, t820: f64, t4368: f64, t25223: f64, t25229: f64, t25235: f64, t25243: f64, t25254: f64, t25276: f64, t25278: f64, t25284: f64, t27244: f64, t27246: f64, t27249: f64, t27251: f64, t27254: f64, t27256: f64) -> (f64, f64) {
    let t27261 = t820 * t25260 * t241;
    let t27262 = t27261 * t4368;
    let t27264 = 7.0_f64 / 144.0_f64 * t25278 - t25284 - t27244 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t27246 - 0.10164000561857065645e-3_f64 * t25235 + t25243 + t25276 - 0.17149607247227894789e-2_f64 * t27249 - 0.10164000561857065645e-3_f64 * t27251 + 0.14291339372689912324e-4_f64 * t27254 + 0.80031500487063509015e-2_f64 * t27256 + t25254 + 0.80031500487063509016e-2_f64 * t25223 + 0.14291339372689912324e-4_f64 * t25229 + 0.85748036236139473944e-3_f64 * t27262;
    (t27262, t27264)
}
