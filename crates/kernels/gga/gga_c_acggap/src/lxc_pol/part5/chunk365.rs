//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 365/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk365<F: Float>(t43: F, t50: F, t103: F, t192: F, t702: F, t705: F, t474: F, t817: F, t292: F, t34: F, t234: F, t821: F, t478: F, t829: F, t296: F, t238: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1272 = t103 * t192;
    let t1279 = 4.0 * t702;
    let t1280 = 0.18311447306006545054e-3 * t705;
    let t1281 = t817 * t474;
    let t1284 = t292 * t34;
    let t1288 = piecewise3(t44, 0.0, -2.0 / 9.0 * t1281 * t234 + 4.0 / 3.0 * t1284 * t821);
    let t1289 = t829 * t478;
    let t1292 = t296 * t34;
    let t1296 = piecewise3(t51, 0.0, -2.0 / 9.0 * t1289 * t238 - 4.0 / 3.0 * t1292 * t821);
    let t1298 = t1288 / 2.0 + t1296 / 2.0;
    (t1272, t1279, t1280, t1281, t1284, t1289, t1292, t1298)
}
