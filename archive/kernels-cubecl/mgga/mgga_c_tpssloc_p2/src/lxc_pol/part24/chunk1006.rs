//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1006/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1006<F: Float>(t3355: F, t432: F, t427: F, t11306: F, t3359: F, t1094: F, t3263: F, t3266: F, t1118: F, t11191: F, t3313: F, t1157: F, t3395: F) -> (F, F, F, F, F) {
    let t11419 = F::cast_from(1.0_f64) / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11421 = t11306 * t3359;
    let t11424 = t1094 * t3263;
    let t11426 = F::cast_from(6.0_f64) * t11424 * t3266;
    let t11427 = t11191 * t1118;
    let t11429 = F::cast_from(6.0_f64) * t3313 * t11427;
    let t11430 = t1157 * t3395;
    (t11420, t11421, t11426, t11429, t11430)
}
