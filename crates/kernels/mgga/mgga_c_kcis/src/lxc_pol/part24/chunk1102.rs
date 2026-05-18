//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1102/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1102<F: Float>(t28059: F, t8069: F, t28045: F, t8072: F, t5047: F, t6486: F, t26896: F, t6693: F, t7748: F, t6613: F, t5077: F, t6496: F) -> (F, F, F, F, F, F, F, F) {
    let t29047 = t28059 * t8069;
    let t29049 = t28045 * t8072;
    let t29051 = t5047 * t6486;
    let t29052 = t26896 * t29051;
    let t29054 = t7748 * t6693;
    let t29056 = t5047 * t6613;
    let t29057 = t7748 * t29056;
    let t29059 = t5077 * t6496;
    (t29047, t29049, t29051, t29052, t29054, t29056, t29057, t29059)
}
