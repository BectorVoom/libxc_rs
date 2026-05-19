//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1198/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1198<F: Float>(t39858: F, t43281: F, t43284: F, t43286: F, t43288: F, t43291: F, t43294: F, t43296: F, t43299: F, t43302: F, t43305: F, t43308: F) -> F {
    let t43310 = -t39858 + F::cast_from(0.47609969197673950971e-2_f64) * t43281 + F::cast_from(0.23804984598836975486e-2_f64) * t43284 + F::cast_from(0.14282990759302185292e-1_f64) * t43286 - F::cast_from(0.16463622957338778997e-1_f64) * t43288 - F::cast_from(0.13099107994629972538e-1_f64) * t43291 + F::cast_from(0.17336443480108537126e0_f64) * t43294 + F::cast_from(0.2600466522016280569e0_f64) * t43296 + F::cast_from(0.2600466522016280569e0_f64) * t43299 + F::cast_from(0.2600466522016280569e0_f64) * t43302 - F::cast_from(0.5200933044032561138e0_f64) * t43305 - F::cast_from(0.2600466522016280569e1_f64) * t43308;
    t43310
}
