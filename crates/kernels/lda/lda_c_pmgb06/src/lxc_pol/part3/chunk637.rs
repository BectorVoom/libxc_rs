//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 637/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk637<F: Float>(t3720: F, t693: F, t681: F, t967: F, t957: F, t963: F, t696: F, t683: F, t978: F, t3662: F, t3672: F, t3678: F, t3700: F, t3701: F, t3707: F, t3713: F, t3714: F, t3719: F) -> (F, F, F, F, F, F, F) {
    let t3721 = t3720 * t693;
    let t3724 = t967 * t681;
    let t3725 = t963 * t957 * t3724;
    let t3727 = F::new(51.94757731704439) * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = F::new(3.5089341735807875) * t696 * t3729;
    let t3732 = F::new(0.0007324578922402618) * t3662 + t3672 - t3678 + t3700 - F::new(1.7544670867903938) * t3701 - t3707 + t3713 + F::new(3.5089341735807875) * t3714 + t3719 - F::new(0.0005493434191801964) * t3721 - t3727 + t3731;
    (t3721, t3724, t3725, t3727, t3729, t3731, t3732)
}
