//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 624/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk624<F: Float>(t1537: F, t4437: F, t3571: F, t3657: F, t3573: F, t3577: F, t3581: F, t3585: F, t3607: F, t3609: F, t3652: F, t3654: F, t3659: F, t3663: F, t3666: F, t3669: F) -> (F, F, F, F) {
    let t4438 = t4437 * t1537;
    let t4443 = 0.68863333333333333333e0 * t3571;
    let t4450 = 0.17365833333333333333e0 * t3657;
    let t4455 = -0.17648625e1 * t3607 + 0.3529725e1 * t3609 + t4443 + 0.34431666666666666666e0 * t3573 - 0.34431666666666666667e0 * t3577 + 0.103295e1 * t3581 - 0.516475e0 * t3585 + 0.31558125e0 * t3652 + 0.6311625e0 * t3654 + t4450 + 0.13892666666666666667e0 * t3659 - 0.34731666666666666667e-1 * t3663 + 0.20839e0 * t3666 - 0.104195e0 * t3669;
    (t4438, t4443, t4450, t4455)
}
