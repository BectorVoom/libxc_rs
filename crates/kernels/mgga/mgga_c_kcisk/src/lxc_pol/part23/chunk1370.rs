//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1370/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1370<F: Float>(t110294: F, t110503: F, t110505: F, t110509: F, t110635: F, t110663: F, t113846: F, t113973: F, t114145: F, t114148: F, t114151: F, t114157: F, t114162: F, t114172: F, t32008: F, t32019: F, t32022: F, t33346: F, t33400: F, t33428: F, t9796: F) -> (F,) {
    let t114176 = -0.13265555555555555555e-1 * t114145 - 0.58958024691358024689e-2 * t114148 + 0.17687407407407407407e-1 * t114151 - 0.20833333333333333334e-1 * t32019 * t33400 - 0.55555555555555555558e-1 * t32022 * t33346 + 0.89351851851851851854e-3 * t114157 + 0.26805555555555555556e-2 * t110663 * t33428 - 0.22109259259259259258e-2 * t114162 + 0.46561250000000000002e-2 * t110635 * t113973 + 0.18518518518518518519e-1 * t110503 - 0.12345679012345679013e-1 * t110505 - t110509 - 0.55555555555555555558e-1 * t110294 * t9796 + 0.99491666666666666664e-2 * t114172 - 0.26805555555555555556e-2 * t32008 * t113846;
    (t114176,)
}
