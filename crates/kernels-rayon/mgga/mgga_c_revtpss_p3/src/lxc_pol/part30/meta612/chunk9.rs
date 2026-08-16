//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2104/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2104(t26024: f64, t5661: f64, t14054: f64, t25986: f64, t2661: f64, t13874: f64, t7271: f64, t94477: f64, t98211: f64, t98213: f64, t98215: f64, t98217: f64, t98218: f64, t98220: f64, t98222: f64, t98224: f64) -> f64 {
    let t98226 = t26024 * t5661;
    let t98227 = 0.40015750243531754508e-2_f64 * t98226;
    let t98229 = t2661 * t25986 * t14054;
    let t98230 = 0.11433071498151929859e-3_f64 * t98229;
    let t98231 = t7271 * t13874;
    let t98233 = 0.17149607247227894789e-2_f64 * t98211 - 0.42874018118069736972e-3_f64 * t98213 + 0.17149607247227894789e-2_f64 * t98215 - t94477 + t98217 - 0.60976381323476959249e-3_f64 * t98218 - 0.90357964994909313586e-5_f64 * t98220 - 0.80031500487063509016e-1_f64 * t98222 - 0.11337795902333997111e-1_f64 * t98224 + t98227 - t98230 + 0.85748036236139473945e-2_f64 * t98231;
    t98233
}
