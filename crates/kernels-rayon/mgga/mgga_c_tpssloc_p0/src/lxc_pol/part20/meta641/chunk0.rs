//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2348/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2348(t42444: f64, t45971: f64, t48140: f64, t2770: f64, t340: f64, t43317: f64, t136: f64, t47746: f64, t908: f64, t2403: f64, t4389: f64, t4386: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48142 = t48140 * t42444 * t45971;
    let t48143 = t340 * t2770;
    let t48145 = t48140 * t48143 * t45971;
    let t48148 = t48140 * t43317 * t45971;
    let t48153 = t136 * t908 * t47746;
    let t48155 = t2403 * t4389;
    let t48156 = 10.0_f64 / 9.0_f64 * t48155;
    let t48157 = t2403 * t4386;
    (t48142, t48145, t48148, t48153, t48155, t48156, t48157)
}
