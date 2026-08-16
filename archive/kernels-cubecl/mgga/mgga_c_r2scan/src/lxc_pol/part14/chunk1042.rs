//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1042/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1042<F: Float>(t2583: F, t3433: F, t2578: F, t2574: F, t146: F, t20946: F, t252: F, t1543: F, t2567: F, t113: F, t2526: F, t494: F) -> (F, F, F, F, F, F) {
    let t26145 = t3433 * t2583;
    let t26150 = t3433 * t2578;
    let t26176 = t3433 * t2574;
    let t26185 = t146 * t20946 * t252;
    let t26186 = t2567 * t1543;
    let t26274 = t2526 * t494 * t113;
    (t26145, t26150, t26176, t26185, t26186, t26274)
}
