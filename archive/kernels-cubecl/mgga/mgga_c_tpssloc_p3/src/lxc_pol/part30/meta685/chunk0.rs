//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2161/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2161<F: Float>(t5303: F, t91100: F, t1339: F, t550: F, t56812: F, t6936: F, t12289: F, t1351: F, t57342: F, t20473: F, t3788: F, t19930: F, t6952: F) -> (F, F, F, F, F) {
    let t97322 = t91100 * t5303;
    let t97326 = t6936 * t1339 * t56812 * t550;
    let t97333 = t6936 * t12289 * t57342 * t1351;
    let t97337 = t6936 * t3788 * t20473 * t1351;
    let t97340 = t6952 * t19930;
    (t97322, t97326, t97333, t97337, t97340)
}
