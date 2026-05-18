//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 467/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk467<F: Float>(t26: F, t3668: F, t3573: F, t3577: F, t3581: F, t3585: F, t3607: F, t3609: F, t3646: F, t3652: F, t3654: F, t3658: F, t3659: F, t3663: F, t3666: F) -> (F, F) {
    let t3669 = t26 * t3668;
    let t3671 = -F::new(0.9494625e0) * t3607 + F::new(0.1898925e1) * t3609 + t3646 + F::new(0.19931111111111111111e0) * t3573 - F::new(0.19931111111111111111e0) * t3577 + F::new(0.59793333333333333334e0) * t3581 - F::new(0.29896666666666666667e0) * t3585 + F::new(0.15358125e0) * t3652 + F::new(0.3071625e0) * t3654 + t3658 + F::new(0.10954222222222222222e0) * t3659 - F::new(0.27385555555555555556e-1) * t3663 + F::new(0.16431333333333333333e0) * t3666 - F::new(0.82156666666666666667e-1) * t3669;
    (t3669, t3671)
}
