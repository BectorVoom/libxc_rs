//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 920/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk920<F: Float>(t30861: F, t7495: F, t7676: F, t7720: F, t2092: F, t7630: F, t2087: F, t1160: F, t30539: F, t1167: F, t151: F, t2116: F, t3668: F) -> (F, F, F, F, F, F, F) {
    let t31619 = t30861 * t7495;
    let t31625 = t7676 * t7720;
    let t31627 = t7630 * t2092;
    let t31629 = t7630 * t2087;
    let t31631 = t1160 * t30539;
    let t31632 = t31631 * t1167;
    let t31643 = t151 * t2116 * t3668;
    (t31619, t31625, t31627, t31629, t31631, t31632, t31643)
}
