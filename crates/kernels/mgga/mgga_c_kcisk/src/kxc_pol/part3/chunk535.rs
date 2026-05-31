//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 535/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk535<F: Float>(t4434: F, t507: F, t1536: F, t1537: F, t3571: F, t3657: F, t3573: F, t3577: F, t3581: F, t3585: F, t3607: F, t3609: F, t3652: F, t3654: F, t3659: F, t3663: F, t3666: F, t3669: F) -> (F, F, F, F, F) {
    let t4435 = F::cast_from(1.0_f64) / t4434;
    let t4436 = t507 * t4435;
    let t4437 = t1536 * t1536;
    let t4438 = t4437 * t1537;
    let t4443 = F::cast_from(0.68863333333333333333e0_f64) * t3571;
    let t4450 = F::cast_from(0.17365833333333333333e0_f64) * t3657;
    let t4455 = -F::cast_from(0.17648625e1_f64) * t3607 + F::cast_from(0.3529725e1_f64) * t3609 + t4443 + F::cast_from(0.34431666666666666666e0_f64) * t3573 - F::cast_from(0.34431666666666666667e0_f64) * t3577 + F::cast_from(0.103295e1_f64) * t3581 - F::cast_from(0.516475e0_f64) * t3585 + F::cast_from(0.31558125e0_f64) * t3652 + F::cast_from(0.6311625e0_f64) * t3654 + t4450 + F::cast_from(0.13892666666666666667e0_f64) * t3659 - F::cast_from(0.34731666666666666667e-1_f64) * t3663 + F::cast_from(0.20839e0_f64) * t3666 - F::cast_from(0.104195e0_f64) * t3669;
    (t4435, t4436, t4437, t4438, t4455)
}
