//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1033/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1033<F: Float>(t25227: F, t2664: F, t2661: F, t2670: F, t7033: F, t2482: F, t27: F, t7043: F, t2677: F, t1941: F, t243: F, t2732: F) -> (F, F, F, F, F, F) {
    let t25228 = t25227 * t2664;
    let t25229 = t2661 * t25228;
    let t25231 = t7033 * t2670;
    let t25234 = t2482 * t7043 * t27;
    let t25235 = t25234 * t2677;
    let t25237 = t1941 * t243;
    let t25238 = t25237 * t2732;
    (t25228, t25229, t25231, t25234, t25235, t25238)
}
