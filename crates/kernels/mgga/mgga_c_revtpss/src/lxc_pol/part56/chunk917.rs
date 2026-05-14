//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 917/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk917<F: Float>(t120000: F, t817: F, t8485: F, t2718: F, t8479: F, t31830: F, t119825: F, t25412: F, t240: F, t27: F, t822: F, t119967: F, t119837: F, t14686: F, t837: F, t119833: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120002 = t120000 * t8485 * t817;
    let t120004 = t8479 * t2718;
    let t120005 = t31830 * t120004;
    let t120006 = t119825 * t25412;
    let t120007 = t120005 * t120006;
    let t120010 = t822 * t27 * t240;
    let t120011 = t119967 * t120010;
    let t120013 = t14686 * t119837 * t837;
    let t120014 = t120011 * t120013;
    let t120016 = t119833 * t120010;
    (t120002, t120004, t120005, t120006, t120007, t120011, t120013, t120014, t120016)
}
