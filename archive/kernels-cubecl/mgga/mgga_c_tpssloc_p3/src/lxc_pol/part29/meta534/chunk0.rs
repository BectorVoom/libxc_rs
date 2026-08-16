//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1917/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1917<F: Float>(t26309: F, t5252: F, t22833: F, t5293: F, t5303: F, t1351: F, t16311: F, t3788: F, t6936: F, t16306: F, t550: F, t1339: F) -> (F, F, F, F, F, F, F, F) {
    let t26310 = t26309 * t5252;
    let t26312 = t22833 * t5293;
    let t26314 = t22833 * t5303;
    let t26318 = t16311 * t1351;
    let t26319 = t3788 * t26318;
    let t26320 = t6936 * t26319;
    let t26322 = t16306 * t550;
    let t26323 = t1339 * t26322;
    (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323)
}
