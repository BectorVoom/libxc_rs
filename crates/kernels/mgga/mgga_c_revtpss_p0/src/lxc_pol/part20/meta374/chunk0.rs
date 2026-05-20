//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1356/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1356<F: Float>(t2723: F, t40262: F, t10666: F, t221: F, t2484: F, t2485: F, t2482: F, t2719: F, t596: F, t10852: F, t2645: F, t10858: F, t10863: F) -> (F, F, F, F, F, F) {
    let t40325 = t2723 * t2723;
    let t40326 = t40262 * t40325;
    let t40333 = t2484 * t2485 * t221 * t10666;
    let t40336 = t2482 * t2719 * t596;
    let t40337 = t40336 * t10852;
    let t40339 = t2645 * t2645;
    let t40340 = t40339 * t2723;
    let t40345 = t10858 * t10863;
    (t40326, t40333, t40337, t40339, t40340, t40345)
}
