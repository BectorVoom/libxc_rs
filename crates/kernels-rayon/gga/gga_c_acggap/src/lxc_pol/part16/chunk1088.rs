//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1088/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1088(t34159: f64, t34162: f64, t34171: f64, t34173: f64, t34176: f64, t36970: f64, t39049: f64, t39052: f64, t39054: f64, t39057: f64, t39060: f64, t39062: f64, t39064: f64, t39069: f64, t39071: f64, t39073: f64, t39075: f64, t39077: f64) -> f64 {
    let t39079 = 0.17149607247227894789e-2_f64 * t39049 - t34159 - 0.38586616306262763275e-1_f64 * t34162 - 0.17149607247227894789e-2_f64 * t39052 + 0.17149607247227894789e-2_f64 * t39054 + t39057 / 128.0_f64 + t39060 / 128.0_f64 - 0.5603125e-1_f64 * t39062 - 0.18868855373762491241e-2_f64 * t39064 - 0.12579236915841660827e-2_f64 * t39069 - 0.34299214494455789578e-2_f64 * t39071 - 0.25724410870841842183e-2_f64 * t39073 + 0.11321313224257494745e-1_f64 * t39075 - t34171 + t34173 + 0.17149607247227894789e-2_f64 * t39077 + t34176 - t36970;
    t39079
}
