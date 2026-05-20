//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1809/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1809<F: Float>(t1976: F, t3270: F, t25464: F, t1096: F, t7135: F, t7160: F, t1982: F, t25460: F) -> (F, F, F) {
    let t25465 = t1976 * t3270;
    let t25466 = t25464 * t25465;
    let t25470 = t7160 * t7135 * t1096;
    let t25473 = t1982 * t25460;
    (t25466, t25470, t25473)
}
