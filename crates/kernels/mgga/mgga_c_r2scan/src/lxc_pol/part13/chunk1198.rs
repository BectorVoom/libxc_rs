//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1198/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1198<F: Float>(t3275: F, t3352: F, t40324: F, t7088: F, t797: F, t3262: F, t3263: F, t114: F, t1543: F, t97: F, t3575: F, t481: F, t7040: F) -> (F, F, F, F) {
    let t40373 = t3275 * t40324 * t3352 / F::new(2.0);
    let t40374 = t797 * t7088;
    let t40377 = F::new(3.0) / F::new(4.0) * t3262 * t3263 * t40374;
    let t40379 = t97 * t1543 * t114;
    let t40381 = F::new(3.0) / F::new(2.0) * t40379 * t3575;
    let t40383 = t7040 * t481;
    (t40373, t40377, t40381, t40383)
}
