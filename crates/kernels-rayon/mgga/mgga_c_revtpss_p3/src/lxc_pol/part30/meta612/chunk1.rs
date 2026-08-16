//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2096/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2096(t13854: f64, t26028: f64, t5697: f64, t94429: f64, t5701: f64, t13995: f64, t98108: f64, t98110: f64, t98112: f64, t98116: f64, t98118: f64, t98120: f64, t98122: f64, t98124: f64) -> f64 {
    let t98126 = t26028 * t13854;
    let t98128 = t94429 * t5697;
    let t98129 = 0.16006300097412701803e-1_f64 * t98128;
    let t98130 = t94429 * t5701;
    let t98131 = 0.40015750243531754508e-2_f64 * t98130;
    let t98132 = t26028 * t13995;
    let t98134 = -0.80031500487063509015e-2_f64 * t98108 + 0.85748036236139473944e-3_f64 * t98110 + 0.34299214494455789578e-2_f64 * t98112 - 0.25724410870841842183e-2_f64 * t98116 + 0.25724410870841842183e-2_f64 * t98118 + 0.17149607247227894789e-2_f64 * t98120 - 0.68598428988911579156e-2_f64 * t98122 - 0.17149607247227894789e-1_f64 * t98124 - 0.42874018118069736972e-3_f64 * t98126 - t98129 + t98131 - 0.85748036236139473945e-2_f64 * t98132;
    t98134
}
