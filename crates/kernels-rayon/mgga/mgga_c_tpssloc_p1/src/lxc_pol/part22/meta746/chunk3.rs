//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2484/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2484(t135: f64, t21537: f64, t973: f64, t21541: f64, t21545: f64, t13995: f64, t18041: f64, t10390: f64, t1041: f64, t21570: f64, t2979: f64, t4582: f64, t48496: f64, t49984: f64, t5909: f64, t62418: f64, t68458: f64, t68466: f64, t68470: f64, t68543: f64, t68547: f64, t68554: f64, t70330: f64, t977: f64) -> (f64, f64, f64, f64) {
    let t70655 = t973 * t135 * t21537;
    let t70660 = t973 * t135 * t21541;
    let t70665 = t973 * t135 * t21545;
    let t70703 = t13995 * t18041;
    let t70707 = t973 * t2979 * t68470 / 72.0_f64 + t973 * t2979 * t68466 / 72.0_f64 + 55.0_f64 / 15552.0_f64 * t1041 * t4582 * t48496 * t70330 + t62418 / 1152.0_f64 - t973 * t977 * t68543 / 12.0_f64 + t973 * t977 * t68547 / 16.0_f64 - t973 * t977 * t68554 / 48.0_f64 - t973 * t977 * t68458 / 48.0_f64 - t49984 * t5909 / 144.0_f64 + t70703 / 1152.0_f64 + 5.0_f64 / 4608.0_f64 * t10390 * t21570;
    (t70655, t70660, t70665, t70707)
}
