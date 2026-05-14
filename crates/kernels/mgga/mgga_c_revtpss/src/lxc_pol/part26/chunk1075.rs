//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1075/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1075<F: Float>(t4132: F, t689: F, t7492: F, t1445: F, t2439: F, t26358: F, t26252: F, t3920: F, t26249: F, t9664: F, t25895: F, t96264: F, t25899: F, t96431: F, t26354: F, t1426: F, t7507: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t96556 = t689 * t7492 * t4132;
    let t96559 = t2439 * t26358 * t1445;
    let t96561 = t26252 * t3920;
    let t96564 = 0.46263278077393568556e-2 * t26249 * t9664;
    let t96565 = t25895 * t96264;
    let t96567 = t25899 * t96431;
    let t96570 = t689 * t26354 * t1445;
    let t96576 = t786 * t7507 * t1426;
    (t96556, t96559, t96561, t96564, t96565, t96567, t96570, t96576)
}
