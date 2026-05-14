//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1037/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1037<F: Float>(t109630: F, t94768: F, t94763: F, t108279: F, t7515: F, t22453: F, t96463: F, t213: F, t30247: F, t689: F, t6896: F, t7492: F, t22399: F, t26265: F, t101970: F, t28154: F) -> (F, F, F, F, F, F, F, F) {
    let t109631 = t94768 * t109630;
    let t109633 = t94763 * t109630;
    let t109647 = t108279 * t7515;
    let t109651 = t96463 * t22453;
    let t109706 = t213 * t30247;
    let t109715 = t689 * t7492 * t6896;
    let t109858 = t26265 * t22399;
    let t109892 = t28154 * t101970;
    (t109631, t109633, t109647, t109651, t109706, t109715, t109858, t109892)
}
