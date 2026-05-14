//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1103/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1103<F: Float>(t5878: F, t997: F, t1084: F, t1180: F, t1181: F, t13503: F, t17221: F, t17223: F, t17228: F, t17230: F, t17232: F, t17234: F, t17236: F, t17238: F, t20764: F, t4289: F, t4735: F, t5799: F) -> (F,) {
    let t22431 = t997 * t5878;
    let t22445 = 0.17149607247227894789e-2 * t17221 + 0.51448821741683684366e-2 * t17223 + 0.85748036236139473944e-3 * t17228 - 0.34299214494455789578e-2 * t17230 + 0.34299214494455789577e-2 * t17232 + 0.16006300097412701803e-1 * t22431 + 0.68598428988911579156e-2 * t17234 + 0.12004725073059526352e-1 * t17236 - 0.34299214494455789578e-2 * t17238 - 0.17149607247227894789e-2 * t1180 * t1181 * t4289 * t5799 + 0.51448821741683684367e-2 * t4735 * t1181 * t20764 * t1084 + 0.24009450146119052705e-1 * t13503;
    (t22445,)
}
