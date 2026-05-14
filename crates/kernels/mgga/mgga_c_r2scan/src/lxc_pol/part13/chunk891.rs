//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 891/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk891<F: Float>(t3366: F, t826: F, t1276: F, t1070: F, t1289: F, t1266: F) -> (F, F, F, F, F, F) {
    let t11050 = t3366 * t826;
    let t11051 = t1276 * t11050;
    let t11052 = 4.0 / 3.0 * t11051;
    let t11053 = t1070 * t1289;
    let t11054 = t1276 * t11053;
    let t11056 = param_eta * t1266;
    (t11050, t11051, t11052, t11053, t11054, t11056)
}
