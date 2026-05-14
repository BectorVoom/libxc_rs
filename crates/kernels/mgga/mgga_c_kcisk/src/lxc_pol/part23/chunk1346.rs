//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1346/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1346<F: Float>(t32176: F, t33384: F, t110242: F, t113578: F, t113579: F, t113582: F, t113584: F, t113598: F, t113601: F, t113604: F, t32019: F, t32026: F, t32096: F, t33434: F, t33439: F, t1339: F, t20005: F, t32045: F) -> (F, F) {
    let t113606 = 0.69444444444444444446e-2 * t33384 * t32176;
    let t113607 = t113578 - 0.71481481481481481484e-2 * t113579 - 0.89351851851851851853e-3 * t113582 - 0.58958024691358024689e-2 * t113584 - 0.41666666666666666668e-1 * t32096 * t33434 - 0.20833333333333333334e-1 * t32096 * t33439 - 0.8041666666666666667e-2 * t32026 * t33439 - 0.23148148148148148148e-2 * t110242 - 0.41666666666666666668e-1 * t32019 * t33434 - 0.20833333333333333334e-1 * t32019 * t33439 + t113598 + 0.33163888888888888888e-2 * t113601 + t113604 + t113606;
    let t113612 = t1339 * t32045 * t20005;
    (t113607, t113612)
}
