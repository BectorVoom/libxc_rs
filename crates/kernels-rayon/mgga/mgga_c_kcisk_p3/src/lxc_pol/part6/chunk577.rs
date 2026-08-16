//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 577/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk577(t338: f64, t7828: f64, t1320: f64, t1310: f64, t1309: f64, t2164: f64, t2170: f64, t3935: f64, t3983: f64, t405: f64, t6155: f64, t6157: f64, t6172: f64, t6197: f64, t8022: f64, t8033: f64, t8037: f64, t8041: f64, t8045: f64, t8050: f64) -> (f64, f64, f64, f64) {
    let t400 = 0.0_f64 < t338;
    let t8054 = piecewise3(t400, t7828, -t7828);
    let t8055 = t1320 * t8054;
    let t8056 = t1310 * t8055;
    let t8059 = 0.5397236614853195164e-1_f64 * t8022 * t405 + 0.35981577432354634426e-1_f64 * t6155 + 0.35981577432354634426e-1_f64 * t6157 * t2164 - 0.10794473229706390328e0_f64 * t6157 * t2170 - t3983 + 0.11993859144118211475e-1_f64 * t6172 - 0.35981577432354634426e-1_f64 * t6197 + 0.23987718288236422951e-1_f64 * t1309 * t8033 - 0.35981577432354634426e-1_f64 * t3935 * t8037 - 0.35981577432354634426e-1_f64 * t1309 * t8041 + 0.17990788716177317213e-1_f64 * t1309 * t8045 + 0.10794473229706390328e0_f64 * t1309 * t8050 - 0.5397236614853195164e-1_f64 * t1309 * t8056;
    (t8054, t8055, t8056, t8059)
}
