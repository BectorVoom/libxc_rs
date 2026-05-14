//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 890/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk890<F: Float>(t33750: F, t944: F, t1219: F, t615: F, t8396: F, t525: F, t847: F, t448: F, t315: F, t2137: F, t33428: F, t1181: F, t5258: F, t604: F, t7575: F, t1165: F, t4930: F, t7351: F) -> (F, F, F, F, F, F, F, F) {
    let t33751 = t33750 * t944;
    let t33778 = t615 * t8396 * t1219;
    let t33787 = t525 * t847;
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33799 = t2137 * t33795;
    let t33802 = t315 * t33428;
    let t33823 = t7575 * t1181 * t604 * t5258;
    let t33827 = t7575 * t1165 * t7351 * t4930;
    (t33751, t33778, t33787, t33796, t33799, t33802, t33823, t33827)
}
