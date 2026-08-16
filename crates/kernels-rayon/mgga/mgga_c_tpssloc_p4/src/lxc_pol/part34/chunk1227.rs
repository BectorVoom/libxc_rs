//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1227/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1227(t105387: f64, t105390: f64, t105393: f64, t105396: f64, t105402: f64, t105404: f64, t105406: f64, t105412: f64, t105415: f64, t84921: f64, t84932: f64, t87387: f64, t87403: f64, t87405: f64, t87432: f64, t87445: f64, t98828: f64, t98830: f64, t98836: f64, t98838: f64) -> f64 {
    let t108309 = -0.50869672678616892475e-1_f64 * t105387 + 0.50869672678616892474e-1_f64 * t105390 - 0.40372756094140390853e-3_f64 * t105393 - 0.18975195364245983701e-1_f64 * t87387 - 5.0_f64 / 32.0_f64 * t105396 + 119.0_f64 / 1152.0_f64 * t87403 - 0.31625325607076639502e-2_f64 * t87405 - 35.0_f64 / 96.0_f64 * t98828 + 7.0_f64 / 48.0_f64 * t98830 + t105402 / 128.0_f64 - t105404 / 128.0_f64 - t105406 / 768.0_f64 - 0.16956557559538964158e-1_f64 * t98836 - 0.67826230238155856633e-1_f64 * t87432 - 0.10173934535723378495e0_f64 * t98838 - t84921 + 0.60559134141210586279e-3_f64 * t87445 - t84932 + t105412 / 64.0_f64 + 0.24223653656484234512e-2_f64 * t105415;
    t108309
}
