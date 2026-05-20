//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1894/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1894<F: Float>(t7284: F, t96370: F, t26234: F, t94886: F, t4132: F, t689: F, t7492: F, t1445: F, t2439: F, t26358: F, t26252: F, t3920: F) -> (F, F, F, F, F) {
    let t96550 = t7284 * t96370;
    let t96552 = t94886 * t26234;
    let t96556 = t689 * t7492 * t4132;
    let t96559 = t2439 * t26358 * t1445;
    let t96561 = t26252 * t3920;
    (t96550, t96552, t96556, t96559, t96561)
}
