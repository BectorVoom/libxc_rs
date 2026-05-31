//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 822/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk822<F: Float>(t3072: F, t311: F, t1072: F, t3062: F, t3066: F, t331: F, t10112: F, t313: F, t1031: F, t1068: F, t1046: F, t3054: F) -> (F, F, F, F, F, F) {
    let t10170 = F::cast_from(1.0_f64) / t3072 / t311;
    let t10182 = t1072 * t3062;
    let t10184 = t331 * t3066;
    let t10187 = F::cast_from(0.14055920378328537299e-1_f64) * t10112 * t313;
    let t10188 = t1068 * t1031;
    let t10190 = t3054 * t1046;
    (t10170, t10182, t10184, t10187, t10188, t10190)
}
