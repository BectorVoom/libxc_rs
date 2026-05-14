//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 452/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk452<F: Float>(t26: F, t3662: F, t1186: F, t3579: F, t3583: F, t3573: F, t3577: F, t3581: F, t3585: F, t3607: F, t3609: F, t3646: F, t3652: F, t3654: F, t3658: F, t3659: F) -> (F, F, F, F, F, F) {
    let t3663 = t26 * t3662;
    let t3665 = t1186 * t3579;
    let t3666 = t26 * t3665;
    let t3668 = t1186 * t3583;
    let t3669 = t26 * t3668;
    let t3671 = -0.9494625e0 * t3607 + 0.1898925e1 * t3609 + t3646 + 0.19931111111111111111e0 * t3573 - 0.19931111111111111111e0 * t3577 + 0.59793333333333333334e0 * t3581 - 0.29896666666666666667e0 * t3585 + 0.15358125e0 * t3652 + 0.3071625e0 * t3654 + t3658 + 0.10954222222222222222e0 * t3659 - 0.27385555555555555556e-1 * t3663 + 0.16431333333333333333e0 * t3666 - 0.82156666666666666667e-1 * t3669;
    (t3663, t3665, t3666, t3668, t3669, t3671)
}
