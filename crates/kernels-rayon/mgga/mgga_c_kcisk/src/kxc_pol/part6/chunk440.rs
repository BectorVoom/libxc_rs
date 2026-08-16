//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 440/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk440(t2900: f64, t47: f64, t2901: f64, t848: f64, t2912: f64, t2918: f64, t2921: f64, t1072: f64, t1079: f64, t1085: f64, t1086: f64, t172: f64, t251: f64, t2849: f64, t2853: f64, t2860: f64, t2882: f64, t2890: f64, t298: f64, t3306: f64, t3312: f64, t3314: f64, t3324: f64, t3329: f64, t3332: f64, t3338: f64, t5: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3342 = t47 * t2900;
    let t3343 = t2901 * t848;
    let t3346 = t2912 * t848;
    let t3349 = t47 * t2918;
    let t3350 = t2901 * t2921;
    let t3353 = -0.70981924444444444442e-3_f64 * t5 * t172 * t251 - 0.34246666666666666666e-1_f64 * t298 * t3306 * t1079 - 2.0_f64 * t3312 * t3314 + 1.0_f64 * t1072 * t3324 + 0.32164683177870697974e2_f64 * t3329 * t3332 + t2849 + t2853 + t2860 - t2882 - t2890 - 0.24415406715670879921e-3_f64 * t5 * t172 * t56 - 0.10843580882781524214e-1_f64 * t298 * t3338 * t1086 - 0.11696446794910408142e1_f64 * t3342 * t3343 + 0.58482233974552040708e0_f64 * t1085 * t3346 + 0.17315755899375863299e2_f64 * t3349 * t3350;
    (t3342, t3343, t3346, t3349, t3350, t3353)
}
