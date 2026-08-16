//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta509<F: Float>(t12566: F, t604: F, t2239: F, t3951: F, t13034: F, t225: F, t10109: F, t1527: F, t13036: F, t4119: F, t828: F, t1484: F, t2678: F) -> (F, F, F, F, F, F, F) {
        let (t46099, t46104, t46452, t46488, t46508, t46565, t46644) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1757::<F>(t12566, t604, t2239, t3951, t13034, t225, t10109, t1527, t13036, t4119, t828, t1484, t2678);
    (t46099, t46104, t46452, t46488, t46508, t46565, t46644)
}
