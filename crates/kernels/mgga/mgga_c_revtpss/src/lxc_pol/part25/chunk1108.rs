//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1108/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1108<F: Float>(t10782: F, t92955: F, t10803: F, t25270: F, t10807: F, t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F, t228: F, t25273: F, t802: F, t25277: F, t2707: F) -> (F, F, F, F, F, F, F) {
    let t92956 = t92955 * t10782;
    let t92958 = t25270 * t10803;
    let t92960 = t25270 * t10807;
    let t92963 = t10744 * t7028 * t2664;
    let t92966 = t2710 * t25240 * t2693;
    let t92968 = t25273 * t228;
    let t92969 = t92968 * t802;
    let t92971 = t25277 * t2707;
    (t92956, t92958, t92960, t92963, t92966, t92969, t92971)
}
