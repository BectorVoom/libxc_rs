//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1220/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1220<F: Float>(t25899: F, t96431: F, t1445: F, t26354: F, t689: F, t1426: F, t7507: F, t786: F, t3917: F, t94701: F, t96204: F, t25878: F, t96242: F) -> (F, F, F, F, F) {
    let t96567 = t25899 * t96431;
    let t96570 = t689 * t26354 * t1445;
    let t96576 = t786 * t7507 * t1426;
    let t96577 = t96576 * t3917;
    let t96584 = F::new(0.51727911450665971904e-3) * t94701 * t96204;
    let t96588 = t25878 * t96242;
    (t96567, t96570, t96577, t96584, t96588)
}
