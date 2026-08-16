//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 895/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk895(t2364: f64, t4609: f64, t8536: f64, t11279: f64, t2372: f64, t8510: f64, t11285: f64, t2487: f64, t28377: f64, t7000: f64, t1421: f64, t22942: f64, t2399: f64, t28911: f64, t28915: f64, t28919: f64, t28925: f64, t28929: f64, t8616: f64) -> f64 {
    let t28933 = t4609 * t2364 * t8536;
    let t28937 = t11279 * t8510 * t2372;
    let t28941 = t11285 * t8510 * t2487;
    let t28944 = t7000 * t28377;
    let t28948 = 0.65704296666666666667e-3_f64 * t1421 * t28911 - 0.22175200125e-2_f64 * t1421 * t28915 + 0.22175200125e-2_f64 * t1421 * t28919 - 12.0_f64 * t2399 * t8616 + 0.295669335e-2_f64 * t1421 * t28925 - 0.19711289e-2_f64 * t1421 * t28929 - 0.19711289e-2_f64 * t1421 * t28933 + 0.49278222499999999999e-2_f64 * t1421 * t28937 - 0.32852148333333333333e-2_f64 * t1421 * t28941 + 0.32852148333333333333e-2_f64 * t1421 * t28944 + 0.39422577999999999999e-2_f64 * t22942;
    t28948
}
