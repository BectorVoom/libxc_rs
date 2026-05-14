//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 837/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk837<F: Float>(t2092: F, t7630: F, t2087: F, t1160: F, t30539: F, t1167: F, t151: F, t2116: F, t3668: F, t409: F, t1103: F, t7746: F, t7637: F, t7709: F, t2113: F, t7610: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31627 = t7630 * t2092;
    let t31629 = t7630 * t2087;
    let t31631 = t1160 * t30539;
    let t31632 = t31631 * t1167;
    let t31643 = t151 * t2116 * t3668;
    let t31644 = t31643 * t409;
    let t31646 = t7746 * t1103;
    let t31658 = t7637 * t7709;
    let t31660 = t7610 * t2113;
    (t31627, t31629, t31631, t31632, t31643, t31644, t31646, t31658, t31660)
}
