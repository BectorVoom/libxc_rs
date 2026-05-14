//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1065/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1065<F: Float>(t20274: F, t16193: F, t16230: F, t16273: F, t16275: F, t16280: F, t16283: F, t16287: F, t16290: F, t19621: F, t19624: F, t19626: F, t19628: F, t19686: F, t19688: F, t19690: F, t19691: F) -> (F, F) {
    let t20275 = 0.17006693853500995666e-1 * t20274;
    let t20317 = -t16193 - t16230 - t16273 + t16275 - t19621 + t19624 + t19626 + t19628 - t16280 + t19686 + t16283 + t16287 - t16290 + t19688 - t19690 + t19691;
    (t20275, t20317)
}
