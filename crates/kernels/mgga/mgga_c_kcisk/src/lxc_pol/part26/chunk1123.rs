//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1123/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1123<F: Float>(t9408: F, t9783: F, t1624: F, t2070: F, t2709: F, t1155: F, t2351: F, t6217: F, t9427: F, t6204: F) -> (F, F, F, F, F) {
    let t33336 = t9408 * t9783;
    let t33338 = t2070 * t1624;
    let t33339 = t2709 * t33338;
    let t33342 = t1155 * t2351;
    let t33343 = t2709 * t33342;
    let t33345 = t9427 * t6217;
    let t33346 = t6204 * t33345;
    (t33336, t33339, t33343, t33345, t33346)
}
