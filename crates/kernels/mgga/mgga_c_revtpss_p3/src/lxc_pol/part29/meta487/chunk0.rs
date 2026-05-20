//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1770/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1770<F: Float>(t28108: F, t77: F, t1470: F, t2242: F, t4181: F, t603: F, t4187: F, t1493: F, t644: F, t4173: F, t607: F, t7705: F) -> (F, F, F, F, F, F, F) {
    let t28109 = t77 * t28108;
    let t28112 = t2242 * t1470;
    let t28116 = t603 * t4181;
    let t28119 = t603 * t4187;
    let t28133 = t77 * t1493 * t644;
    let t28141 = t4173 * t607;
    let t28147 = t77 * t7705 * t644;
    (t28109, t28112, t28116, t28119, t28133, t28141, t28147)
}
