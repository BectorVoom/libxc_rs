//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 763/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk763<F: Float>(t2133: F, t6303: F, t120: F, t122: F, t135: F, t273: F, t57: F, t2096: F, t784: F, t23: F, t271: F, t6077: F) -> (F, F, F, F) {
    let t6304 = t2133 * t6303;
    let t6310 = F::cast_from(0.92480845007273388189e0_f64) * t120 * t122 * t273 * t57 * t135;
    let t6311 = t2096 * t784;
    let t6314 = F::new(1.0) / t23 / t6077 / t271;
    (t6304, t6310, t6311, t6314)
}
