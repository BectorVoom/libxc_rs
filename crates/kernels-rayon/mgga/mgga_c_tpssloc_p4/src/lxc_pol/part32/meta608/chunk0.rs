//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2005/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2005(t225: f64, t814: f64, t6648: f64, t81612: f64, t22715: f64, t6551: f64, t6640: f64, t117: f64, t4179: f64, t6559: f64, t229: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81632 = t22715 * t6551;
    let t81633 = t81632 * t6640;
    let t81640 = t6559 * t4179 * t117;
    let t81651 = t6559 * t229 * t268;
    (t81613, t81615, t81632, t81633, t81640, t81651)
}
