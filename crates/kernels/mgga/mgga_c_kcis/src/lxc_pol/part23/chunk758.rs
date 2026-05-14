//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 758/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk758<F: Float>(t23: F, t821: F, t6: F, t107: F, t2621: F, t9: F, t7: F, t118: F, t2474: F, t5: F, t22: F, t4864: F, t41: F, t85: F, t8565: F, t11: F) -> (F, F, F, F, F, F) {
    let t13581 = 1.0 / t23 / t821;
    let t13582 = t6 * t13581;
    let t13583 = t107 * t13582;
    let t13587 = 1.0 / t9 / t2621;
    let t13588 = t7 * t13587;
    let t13589 = t118 * t13588;
    let t13716 = t5 * t2474;
    let t13948 = t22 * t4864;
    let t14249 = t85 * t8565 * t41;
    let t14954 = t11 * t41;
    (t13583, t13589, t13716, t13948, t14249, t14954)
}
