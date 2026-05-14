//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 659/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk659<F: Float>(t1186: F, t5601: F, t5744: F, t3573: F, t3646: F, t3658: F, t3659: F, t5668: F, t5673: F, t5678: F, t5682: F, t5691: F, t5693: F, t5731: F, t5733: F, t5736: F, t5739: F, t5742: F) -> (F, F, F) {
    let t5745 = t1186 * t5601;
    let t5746 = t5744 * t5745;
    let t5748 = -0.9494625e0 * t5691 + 0.1898925e1 * t5693 + t3646 + 0.99655555555555555557e-1 * t3573 + 0.99655555555555555557e-1 * t5668 - 0.19931111111111111111e0 * t5673 + 0.59793333333333333334e0 * t5678 - 0.59793333333333333334e0 * t5682 + 0.15358125e0 * t5731 + 0.3071625e0 * t5733 + t3658 + 0.54771111111111111111e-1 * t3659 + 0.54771111111111111111e-1 * t5736 - 0.27385555555555555556e-1 * t5739 + 0.16431333333333333333e0 * t5742 - 0.16431333333333333333e0 * t5746;
    (t5745, t5746, t5748)
}
