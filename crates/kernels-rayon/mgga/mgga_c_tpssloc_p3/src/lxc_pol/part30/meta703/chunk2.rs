//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2287/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2287(t40: f64, t5842: f64, t1933: f64, t23479: f64, t17701: f64, t17877: f64, t18021: f64, t1937: f64, t1941: f64, t23419: f64, t28525: f64, t28582: f64, t378: f64, t4579: f64, t6722: f64, t83117: f64, t83215: f64, t88422: f64, t88425: f64, t88428: f64, t88440: f64, t88453: f64, t88513: f64) -> (f64, f64) {
    let t99645 = t40 * t5842;
    let t99647 = t1933 * t99645 * t23479;
    let t99654 = -t88422 - t88425 - t88428 + t88513 * t4579 / 1152.0_f64 + t88440 + t88453 - t83215 * t17701 / 2304.0_f64 + t23419 * t18021 / 2304.0_f64 + t17877 * t1941 * t378 / 1536.0_f64 - 0.10093189023535097714e-3_f64 * t99647 - 0.80745512188280781712e-3_f64 * t6722 * t28525 * t1937 - 0.10093189023535097714e-3_f64 * t83117 * t28582;
    (t99645, t99654)
}
