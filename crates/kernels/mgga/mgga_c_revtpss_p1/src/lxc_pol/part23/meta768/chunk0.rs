//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2568/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2568<F: Float>(t3599: F, t56802: F, t3609: F, t3623: F, t53739: F, t13127: F, t1214: F, t3611: F, t12831: F, t17395: F, t13148: F, t17728: F, t460: F, t489: F) -> (F, F, F, F, F, F, F, F) {
    let t56803 = t56802 * t3599;
    let t56806 = t56802 * t3609;
    let t56878 = t3623 * t53739;
    let t56879 = t13127 * t56878;
    let t56947 = t3611 * t1214;
    let t56953 = t12831 * t17395;
    let t56997 = t13148 * t56878;
    let t57005 = t460 * t489 * t17728;
    (t56803, t56806, t56878, t56879, t56947, t56953, t56997, t57005)
}
