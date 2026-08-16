//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 869/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk869(t1167: f64, t3638: f64, t3641: f64, t1191: f64, t12911: f64, t3677: f64, t1192: f64, t3671: f64, t3639: f64, t1190: f64, t3679: f64, t330: f64, t3721: f64) -> (f64, f64, f64, f64, f64) {
    let t13048 = t1167 * t3638;
    let t13050 = 6.0_f64 * t13048 * t3641;
    let t13051 = t12911 * t1191;
    let t13053 = 6.0_f64 * t3677 * t13051;
    let t13054 = t1192 * t3671;
    let t13056 = 6.0_f64 * t3639 * t13054;
    let t13058 = t3671 * t3679 * t1190;
    let t13060 = 0.48245472966453314466e2_f64 * t3677 * t13058;
    let t13064 = 1.0_f64 / t3721 / t330;
    (t13050, t13053, t13056, t13060, t13064)
}
