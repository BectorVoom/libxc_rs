//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2168/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2168(t14693: f64, t25270: f64, t14927: f64, t27261: f64, t93001: f64, t92996: f64, t92998: f64, t93000: f64, t99042: f64, t99044: f64, t99046: f64, t99048: f64, t99050: f64, t99052: f64) -> f64 {
    let t99054 = t25270 * t14693;
    let t99056 = t27261 * t14927;
    let t99058 = 0.1219527626469539185e-2_f64 * t93001;
    let t99059 = t99042 + 0.2032800112371413129e-4_f64 * t99044 + t99046 / 8.0_f64 + t99048 / 16.0_f64 - t92996 - 35.0_f64 / 216.0_f64 * t99050 + 0.17149607247227894789e-2_f64 * t99052 + 0.34299214494455789578e-2_f64 * t99054 + 0.25724410870841842183e-2_f64 * t99056 - t92998 + t93000 - t99058;
    t99059
}
