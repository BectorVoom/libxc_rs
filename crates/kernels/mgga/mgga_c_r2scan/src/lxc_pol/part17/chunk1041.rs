//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1041/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1041<F: Float>(t2583: F, t3433: F, t2578: F, t2574: F, t146: F, t20946: F, t252: F, t545: F, t7600: F, t6091: F, t978: F, t2145: F, t2832: F) -> (F, F, F, F, F, F, F) {
    let t26145 = t3433 * t2583;
    let t26150 = t3433 * t2578;
    let t26176 = t3433 * t2574;
    let t26185 = t146 * t20946 * t252;
    let t26278 = t545 * t7600;
    let t26282 = t146 * t6091 * t978;
    let t27067 = t146 * t2145 * t2832;
    (t26145, t26150, t26176, t26185, t26278, t26282, t27067)
}
