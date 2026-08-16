//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1306/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1306(t27221: f64, t76613: f64, t23267: f64, t7025: f64, t106053: f64, t106061: f64, t106063: f64, t106065: f64, t92996: f64, t92998: f64, t93000: f64, t93008: f64, t93013: f64, t99035: f64, t99044: f64, t99050: f64) -> f64 {
    let t113214 = t27221 * t76613;
    let t113217 = t7025 * t23267;
    let t113219 = -0.34299214494455789577e-3_f64 * t106053 - 0.34013387707001991332e-1_f64 * t99035 + 0.17149607247227894789e-3_f64 * t106061 + 0.60023625365297631762e-2_f64 * t106063 - 0.12004725073059526352e-1_f64 * t106065 + 0.60984003371142393869e-4_f64 * t99044 - t92996 + 3.0_f64 / 16.0_f64 * t113214 - 35.0_f64 / 72.0_f64 * t99050 - t92998 + t93000 + t93008 - t113217 / 48.0_f64 - t93013;
    t113219
}
