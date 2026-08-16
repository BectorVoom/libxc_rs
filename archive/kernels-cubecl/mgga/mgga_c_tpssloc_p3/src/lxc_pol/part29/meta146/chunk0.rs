//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 800/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk800<F: Float>(t1055: F, t3206: F, t1052: F, t1066: F, t3021: F, t3023: F, t3026: F, t3167: F, t3169: F, t3176: F, t388: F, t1068: F) -> (F, F, F) {
    let t3207 = t1055 * t3206;
    let t3209 = F::cast_from(2.0_f64) * t1052 * t3176 - t1052 * t3207 - F::cast_from(2.0_f64) * t1066 * t3026 - F::cast_from(2.0_f64) * t1066 * t3169 + t3021 * t388 + F::cast_from(2.0_f64) * t3023 * t388 + t3167 * t388;
    let t3213 = t1068 * t1068;
    (t3207, t3209, t3213)
}
