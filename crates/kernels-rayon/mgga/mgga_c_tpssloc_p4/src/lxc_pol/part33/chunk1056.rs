//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1056/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1056(t22228: f64, t3403: f64, t1164: f64, t1147: f64, t1156: f64, t21938: f64, t11282: f64, t21906: f64, t11285: f64, t4869: f64, t6102: f64, t21726: f64, t21728: f64, t21730: f64, t21732: f64, t21897: f64, t21901: f64, t21990: f64, t21993: f64) -> (f64, f64, f64, f64, f64) {
    let t22229 = t22228 * t3403;
    let t22231 = 0.10389515463408878255e3_f64 * t1164 * t22229;
    let t22233 = t1147 * t21938 * t1156;
    let t22235 = 0.5848223622634646207e0_f64 * t1164 * t22233;
    let t22236 = t11282 * t21906;
    let t22237 = t22236 * t11285;
    let t22239 = 0.10254018858216406658e4_f64 * t1164 * t22237;
    let t22241 = 0.17544670867903938621e1_f64 * t4869 * t6102;
    let t22242 = t22231 - t22235 - t22239 + t21726 - t21897 + t21901 - t21730 - t22241 - t21728 - t21990 + t21732 + t21993;
    (t22231, t22235, t22239, t22241, t22242)
}
