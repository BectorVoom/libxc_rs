//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1238/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1238<F: Float>(t12987: F, t480: F, t12629: F, t482: F, t371: F, t372: F, t127: F, t3672: F, t3671: F, t140: F, t3693: F, t1222: F) -> (F, F, F, F, F, F) {
    let t12988 = t12987 * t480;
    let t12989 = t482 * t12629;
    let t12991 = t371 * t372 * t12989;
    let t12995 = t371 * t127 * t3672;
    let t12996 = t3671 * t12995;
    let t12998 = t140 * t3693;
    let t12999 = t1222 * t12998;
    (t12988, t12989, t12991, t12995, t12996, t12999)
}
