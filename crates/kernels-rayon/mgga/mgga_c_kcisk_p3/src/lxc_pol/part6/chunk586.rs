//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 586/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk586(t3484: f64, t8176: f64, t3482: f64, t1341: f64, t7740: f64, t1340: f64, t1339: f64, t1220: f64, t5610: f64, t8064: f64, t8075: f64, t8080: f64, t8084: f64, t8087: f64, t8091: f64, t8095: f64, t8165: f64, t8173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8177 = t3484 * t8176;
    let t8178 = t3482 * t8177;
    let t8180 = t1341 * t7740;
    let t8181 = t1340 * t8180;
    let t8182 = t1339 * t8181;
    let t8184 = 0.49745833333333333332e-2_f64 * t8075 - 0.33163888888888888888e-2_f64 * t8080 - 0.55273148148148148147e-3_f64 * t8084 + 0.33163888888888888888e-2_f64 * t8087 + 0.16581944444444444444e-2_f64 * t8091 + 0.27636574074074074073e-2_f64 * t8095 + 0.24872916666666666666e-2_f64 * t8165 + 0.22109259259259259258e-2_f64 * t5610 + 0.193e0_f64 * t1220 * t8064 - 0.33163888888888888888e-2_f64 * t8173 + 0.22109259259259259258e-2_f64 * t8178 - 0.33163888888888888888e-2_f64 * t8182;
    (t8177, t8178, t8180, t8181, t8182, t8184)
}
