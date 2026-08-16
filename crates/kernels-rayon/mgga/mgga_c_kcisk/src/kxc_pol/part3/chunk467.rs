//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 467/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk467(t26: f64, t3668: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64, t3607: f64, t3609: f64, t3646: f64, t3652: f64, t3654: f64, t3658: f64, t3659: f64, t3663: f64, t3666: f64) -> (f64, f64) {
    let t3669 = t26 * t3668;
    let t3671 = -0.9494625e0_f64 * t3607 + 0.1898925e1_f64 * t3609 + t3646 + 0.19931111111111111111e0_f64 * t3573 - 0.19931111111111111111e0_f64 * t3577 + 0.59793333333333333334e0_f64 * t3581 - 0.29896666666666666667e0_f64 * t3585 + 0.15358125e0_f64 * t3652 + 0.3071625e0_f64 * t3654 + t3658 + 0.10954222222222222222e0_f64 * t3659 - 0.27385555555555555556e-1_f64 * t3663 + 0.16431333333333333333e0_f64 * t3666 - 0.82156666666666666667e-1_f64 * t3669;
    (t3669, t3671)
}
