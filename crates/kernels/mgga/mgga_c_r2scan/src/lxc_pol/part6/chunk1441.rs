//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1441/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1441<F: Float>(t44: F, t2727: F, t6487: F, t2236: F, t7391: F, t2155: F, t25193: F, t2706: F, t409: F, t23842: F, t23845: F, t23851: F, t23854: F, t2509: F, t2512: F, t40: F, t415: F, t4905: F, t4913: F, t7059: F, t7062: F, t7067: F, t903: F, t99: F, zeta_threshold: F) -> (F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t27110 = t6487 * t2727;
    let t27115 = t2236 * t7391;
    let t27125 = t2155 * t25193;
    let t27144 = 40.0 * t2706 * t409;
    let t27146 = piecewise3(t45, 0.0, 40.0 / 81.0 * t7059 * t4905 - 20.0 / 9.0 * t7062 * t23842 - 10.0 / 9.0 * t2509 * t23845 + 20.0 / 3.0 * t7067 * t415 - 20.0 * t2512 * t23851 + 20.0 / 3.0 * t2512 * t23854 + 10.0 / 9.0 * t903 * t4913 - 20.0 * t99 * t40 + t27144);
    (t27110, t27115, t27125, t27146)
}
