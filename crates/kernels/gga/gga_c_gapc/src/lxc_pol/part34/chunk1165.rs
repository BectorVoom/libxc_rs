//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1165/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1165<F: Float>(t11749: F, t33643: F, t933: F, t11790: F, t3367: F, t6188: F, t11794: F, t7927: F, t9554: F, t126: F, t671: F, t128: F, t314: F) -> (F, F, F, F, F) {
    let t33645 = t933 * t33643 * t11749;
    let t33648 = t11790 * t3367 * t6188;
    let t33653 = t11794 * t7927 * t9554;
    let t33655 = t126 * t671;
    let t33657 = t314 * t128;
    (t33645, t33648, t33653, t33655, t33657)
}
