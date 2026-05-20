//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1310/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1310<F: Float>(t1209: F, t96873: F, t1269: F, t7642: F, t8945: F, t1243: F, t26884: F, t12941: F, t7618: F, t13068: F, t7617: F, t26873: F, t3704: F) -> (F, F, F, F, F, F) {
    let t97078 = t1209 * t96873;
    let t97081 = t7642 * t1269;
    let t97082 = t97081 * t8945;
    let t97095 = t1243 * t26884;
    let t97112 = t7618 * t12941;
    let t97120 = t13068 * t7617;
    let t97125 = t26873 * t3704;
    (t97078, t97082, t97095, t97112, t97120, t97125)
}
