//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2269/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2269(t97989: f64, t98039: f64, t98090: f64, t99067: f64, t1634: f64, t607: f64, t1065: f64, t5392: f64, t17686: f64, t1927: f64, t23327: f64, t23329: f64, t25424: f64, t25429: f64, t25430: f64, t25442: f64, t25738: f64, t25815: f64, t28701: f64, t28702: f64, t4337: f64, t7553: f64, t82342: f64, t82402: f64, t82417: f64, t88004: f64, t88050: f64, t88069: f64, t88075: f64, t88083: f64, t88089: f64, t88112: f64) -> (f64, f64, f64, f64) {
    let t99069 = t97989 + t98039 + t98090 + t99067;
    let t99070 = t1634 * t607;
    let t99099 = t5392 * t1065;
    let t99104 = -0.73108180748810063845e-2_f64 * t25429 * t88112 * t4337 * t99070 - 0.54831135561607547884e-2_f64 * t23327 * t88004 * t7553 - 0.54831135561607547884e-2_f64 * t23327 * t88089 * t7553 - 0.54831135561607547884e-2_f64 * t23327 * t88050 * t25815 + 0.14621636149762012769e-1_f64 * t82402 * t28702 - 0.3289868133696452873e-1_f64 * t1927 * t25442 * t25738 - 0.10966227112321509577e-1_f64 * t23327 * t88050 * t25424 - 0.54831135561607547884e-2_f64 * t23327 * t82417 * t28701 + 0.16449340668482264365e-1_f64 * t23327 * t23329 * t25430 * t17686 - t88069 - t88075 - t88083 + 0.54831135561607547884e-2_f64 * t23327 * t23329 * t82342 * t99099;
    (t99069, t99070, t99099, t99104)
}
