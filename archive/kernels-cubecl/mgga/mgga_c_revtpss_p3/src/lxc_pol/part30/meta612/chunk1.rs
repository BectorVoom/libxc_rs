//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2096/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2096<F: Float>(t13854: F, t26028: F, t5697: F, t94429: F, t5701: F, t13995: F, t98108: F, t98110: F, t98112: F, t98116: F, t98118: F, t98120: F, t98122: F, t98124: F) -> F {
    let t98126 = t26028 * t13854;
    let t98128 = t94429 * t5697;
    let t98129 = F::cast_from(0.16006300097412701803e-1_f64) * t98128;
    let t98130 = t94429 * t5701;
    let t98131 = F::cast_from(0.40015750243531754508e-2_f64) * t98130;
    let t98132 = t26028 * t13995;
    let t98134 = -F::cast_from(0.80031500487063509015e-2_f64) * t98108 + F::cast_from(0.85748036236139473944e-3_f64) * t98110 + F::cast_from(0.34299214494455789578e-2_f64) * t98112 - F::cast_from(0.25724410870841842183e-2_f64) * t98116 + F::cast_from(0.25724410870841842183e-2_f64) * t98118 + F::cast_from(0.17149607247227894789e-2_f64) * t98120 - F::cast_from(0.68598428988911579156e-2_f64) * t98122 - F::cast_from(0.17149607247227894789e-1_f64) * t98124 - F::cast_from(0.42874018118069736972e-3_f64) * t98126 - t98129 + t98131 - F::cast_from(0.85748036236139473945e-2_f64) * t98132;
    t98134
}
