//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1129/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1129<F: Float>(t1972: F, t4857: F, t1659: F, t7131: F, t25515: F, t4890: F, t3299: F, t3317: F, t1967: F, t816: F, t1014: F, t65: F) -> (F, F, F, F, F, F, F) {
    let t27479 = t4857 * t1972;
    let t27489 = t1659 * t7131;
    let t27492 = t25515 * t4890;
    let t27493 = t3299 * t27492;
    let t27498 = t3317 * t27492;
    let t27526 = t1967 * t816;
    let t27527 = t65 * t1014;
    (t27479, t27489, t27492, t27493, t27498, t27526, t27527)
}
