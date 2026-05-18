//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 766/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk766<F: Float>(t9296: F, t9311: F, t160: F, t167: F, t251: F, t88: F, t41: F, t4879: F, t4585: F, t85: F, t3250: F, t119: F) -> (F, F, F, F, F, F, F) {
    let t9312 = t9296 + t9311;
    let t9323 = t167 * t160;
    let t9526 = t88 * t251;
    let t10138 = t4879 * t41;
    let t10269 = t85 * t4585;
    let t10338 = t85 * t3250 * t41;
    let t10470 = t85 * t119 * t251;
    (t9312, t9323, t9526, t10138, t10269, t10338, t10470)
}
