//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2016/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2016(t103273: f64, t103276: f64, t103280: f64, t103283: f64, t106022: f64, t106024: f64, t95671: f64, t98976: f64, t98979: f64, t99002: f64, t99004: f64, t99009: f64) -> f64 {
    let t110393 = 0.10164000561857065645e-2_f64 * t106022 - 0.80031500487063509015e-1_f64 * t106024 - 0.14457274399185490173e-3_f64 * t98976 + 0.2032800112371413129e-4_f64 * t98979 + t103273 + t103276 - t103280 + 0.10841600599314203355e-2_f64 * t99002 - t95671 + t99004 + t103283 - 0.18140473443734395377e0_f64 * t99009;
    t110393
}
