//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1209/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1209(t1115: f64, t39197: f64, t42878: f64, t39190: f64, t42882: f64, t11002: f64, t3229: f64, t3269: f64, t11336: f64, t3232: f64, t3270: f64, t3579: f64, t41190: f64) -> (f64, f64, f64, f64, f64) {
    let t44061 = 15.0_f64 / 4.0_f64 * t39197 * t1115 * t42878;
    let t44064 = 135.0_f64 / 32.0_f64 * t39190 * t1115 * t42882;
    let t44066 = t11002 * t1115 * t3229;
    let t44068 = 5.0_f64 / 16.0_f64 * t3269 * t44066;
    let t44070 = t3270 * t11336 * t3232;
    let t44072 = t3269 * t44070 / 4.0_f64;
    let t44074 = 5.0_f64 / 8.0_f64 * t3579 * t41190;
    (t44061, t44064, t44068, t44072, t44074)
}
