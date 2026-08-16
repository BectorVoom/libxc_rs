//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2124/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2124(t18622: f64, t25245: f64, t5989: f64, t92978: f64, t18634: f64, t27261: f64, t18334: f64, t25270: f64, t25277: f64, t5985: f64, t93021: f64, t93035: f64, t99066: f64, t99070: f64, t99074: f64, t99078: f64, t99086: f64) -> f64 {
    let t106080 = t25245 * t18622;
    let t106082 = t92978 * t5989;
    let t106085 = t27261 * t18634;
    let t106088 = t25270 * t18334;
    let t106090 = t25277 * t5985;
    let t106092 = -t93021 - 0.25410001404642664113e-4_f64 * t106080 - 7.0_f64 / 48.0_f64 * t106082 - 0.80031500487063509015e-1_f64 * t99066 - t99070 + t99074 - t99078 + t99086 + 0.17149607247227894789e-2_f64 * t106085 + 0.27104001498285508387e-3_f64 * t93035 + 0.34299214494455789578e-2_f64 * t106088 + 7.0_f64 / 144.0_f64 * t106090;
    t106092
}
