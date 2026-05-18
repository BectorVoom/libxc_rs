//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1120/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1120<F: Float>(t28765: F, t5654: F, t6151: F, t6188: F, t7969: F, t6176: F, t18210: F, t8212: F, t7978: F, t8225: F, t7974: F, t8218: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28766 = t28765 * t5654;
    let t28767 = t6151 * t28766;
    let t28771 = t7969 * t6188;
    let t28772 = t6176 * t28771;
    let t28778 = t18210 * t8212;
    let t28779 = t7978 * t28778;
    let t28781 = t18210 * t8225;
    let t28782 = t7978 * t28781;
    let t28784 = t8218 * t7974;
    (t28766, t28767, t28771, t28772, t28778, t28779, t28781, t28782, t28784)
}
