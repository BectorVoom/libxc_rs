//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1250/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1250(t27484: f64, t8144: f64, t28387: f64, t61287: f64, t16642: f64, t4160: f64, t94425: f64, t27455: f64, t28369: f64, t94398: f64, t94402: f64, t98242: f64, t98378: f64, t98381: f64, t98383: f64, t98387: f64, t98388: f64) -> (f64, f64) {
    let t98390 = t8144 * t27484;
    let t98392 = t28387 * t61287;
    let t98396 = t4160 * t94425 * t16642;
    let t98400 = -0.33163888888888888888e-2_f64 * t98378 + t98381 - t98383 + 0.46336805555555555556e-3_f64 * t28369 * t27455 - t98387 + 0.12356481481481481481e-2_f64 * t98388 + 0.15445601851851851852e-3_f64 * t98390 - 0.12378114784505208333e-4_f64 * t98392 * t98242 - 0.22109259259259259258e-2_f64 * t98396 - 0.7722800925925925926e-4_f64 * t94398 - 0.10297067901234567901e-3_f64 * t94402;
    (t98396, t98400)
}
