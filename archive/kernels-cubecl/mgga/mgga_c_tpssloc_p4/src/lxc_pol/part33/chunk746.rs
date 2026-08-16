//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 746/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk746<F: Float>(t112: F, t2022: F, t33: F, t3953: F, t1437: F, t79: F, t72: F, t1410: F, t605: F, t1409: F, t6500: F, t6503: F) -> (F, F, F, F, F, F) {
    let t7010 = t2022 * t112;
    let t7428 = t3953 * t33;
    let t7431 = t79 * t1437;
    let t7432 = t72 * t7431;
    let t7435 = t605 * t1410;
    let t7440 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6500 * t1409 + t6503;
    (t7010, t7428, t7431, t7432, t7435, t7440)
}
