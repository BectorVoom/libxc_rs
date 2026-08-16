//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 535/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk535(t4434: f64, t507: f64, t1536: f64, t1537: f64, t3571: f64, t3657: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64, t3607: f64, t3609: f64, t3652: f64, t3654: f64, t3659: f64, t3663: f64, t3666: f64, t3669: f64) -> (f64, f64, f64, f64, f64) {
    let t4435 = 1.0_f64 / t4434;
    let t4436 = t507 * t4435;
    let t4437 = t1536 * t1536;
    let t4438 = t4437 * t1537;
    let t4443 = 0.68863333333333333333e0_f64 * t3571;
    let t4450 = 0.17365833333333333333e0_f64 * t3657;
    let t4455 = -0.17648625e1_f64 * t3607 + 0.3529725e1_f64 * t3609 + t4443 + 0.34431666666666666666e0_f64 * t3573 - 0.34431666666666666667e0_f64 * t3577 + 0.103295e1_f64 * t3581 - 0.516475e0_f64 * t3585 + 0.31558125e0_f64 * t3652 + 0.6311625e0_f64 * t3654 + t4450 + 0.13892666666666666667e0_f64 * t3659 - 0.34731666666666666667e-1_f64 * t3663 + 0.20839e0_f64 * t3666 - 0.104195e0_f64 * t3669;
    (t4435, t4436, t4437, t4438, t4455)
}
