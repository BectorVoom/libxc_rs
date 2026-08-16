//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1217/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1217(t34293: f64, t37013: f64, t37014: f64, t37016: f64, t37017: f64, t39141: f64, t39143: f64, t39145: f64, t39147: f64, t39151: f64, t39155: f64, t39160: f64, t39162: f64, t39167: f64, t39169: f64, t39171: f64, t39173: f64, t39176: f64) -> f64 {
    let t41510 = -0.18868855373762491241e-2_f64 * t39141 + 0.13719685797782315831e-1_f64 * t39143 + 0.68598428988911579156e-2_f64 * t39145 - 0.34299214494455789578e-2_f64 * t39147 - 0.12579236915841660827e-2_f64 * t39151 - 0.15724046144802076034e-2_f64 * t39155 + 0.62896184579208304138e-3_f64 * t39160 - 0.12862205435420921092e-1_f64 * t39162 - 0.94344276868812456207e-3_f64 * t39167 - t39169 / 24.0_f64 - t39171 / 48.0_f64 - 0.80031500487063509015e-2_f64 * t39173 + 0.64025200389650807212e-1_f64 * t34293 + t37013 - t37014 + t37016 + t37017 - 0.21437009059034868486e-2_f64 * t39176;
    t41510
}
