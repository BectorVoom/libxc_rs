//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 999/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk999<F: Float>(t26930: F, t6720: F, t1096: F, t6724: F, t1021: F, t6728: F, t6732: F, t29045: F, t29047: F, t29049: F, t29052: F, t29054: F, t29057: F, t29060: F, t29063: F, t29065: F, t29067: F, t29069: F, t29071: F) -> (F, F, F, F, F) {
    let t29073 = t26930 * t6720;
    let t29075 = t1096 * t6724;
    let t29077 = t1021 * t6728;
    let t29079 = t1021 * t6732;
    let t29081 = t29045 / 16.0 - t29047 / 8.0 + t29049 / 12.0 + t29052 / 8.0 - t29054 / 12.0 - t29057 / 16.0 - t29060 / 72.0 + t29063 / 24.0 - t29065 / 128.0 + t29067 / 64.0 - t29069 / 48.0 - t29071 / 64.0 + t29073 / 48.0 + t29075 / 128.0 - t29077 / 288.0 - t29079 / 96.0;
    (t29073, t29075, t29077, t29079, t29081)
}
