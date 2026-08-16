//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1160/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1160(t33960: f64, t33962: f64, t30226: f64, t30234: f64, t30240: f64, t30249: f64, t30251: f64, t30253: f64, t32397: f64, t32398: f64, t32401: f64, t32403: f64, t32404: f64, t33956: f64, t33966: f64, t33974: f64, t33979: f64) -> f64 {
    let t36876 = 0.7640625e-2_f64 * t33960;
    let t36877 = 11.0_f64 / 96.0_f64 * t33962;
    let t36887 = -0.42874018118069736972e-2_f64 * t33956 - t36876 + t36877 - t33966 / 64.0_f64 + 0.34299214494455789578e-2_f64 * t30226 + t32397 + t32398 - 0.17149607247227894789e-2_f64 * t30234 + t32401 + 0.42874018118069736972e-3_f64 * t30240 + t32403 - t32404 - 0.18140473443734395377e0_f64 * t30249 - 0.24009450146119052704e-1_f64 * t30251 + 0.17149607247227894789e-1_f64 * t30253 - 0.34299214494455789578e-2_f64 * t33974 + 0.31448092289604152069e-3_f64 * t33979;
    t36887
}
