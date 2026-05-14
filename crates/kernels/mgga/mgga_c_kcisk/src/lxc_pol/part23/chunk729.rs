//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 729/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk729<F: Float>(t1536: F, t2293: F, t3573: F, t3659: F, t4443: F, t4450: F, t5668: F, t5673: F, t5678: F, t5682: F, t5691: F, t5693: F, t5731: F, t5733: F, t5736: F, t5739: F, t5742: F, t5746: F) -> (F, F) {
    let t6523 = t2293 * t1536;
    let t6540 = -0.17648625e1 * t5691 + 0.3529725e1 * t5693 + t4443 + 0.17215833333333333333e0 * t3573 + 0.17215833333333333333e0 * t5668 - 0.34431666666666666667e0 * t5673 + 0.103295e1 * t5678 - 0.103295e1 * t5682 + 0.31558125e0 * t5731 + 0.6311625e0 * t5733 + t4450 + 0.69463333333333333333e-1 * t3659 + 0.69463333333333333333e-1 * t5736 - 0.34731666666666666667e-1 * t5739 + 0.20839e0 * t5742 - 0.20839e0 * t5746;
    (t6523, t6540)
}
