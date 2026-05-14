//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1412/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1412<F: Float>(t7601: F, t9293: F, t10127: F, t20286: F, t25872: F, t2719: F, t30252: F, t30254: F, t30258: F, t30260: F, t30283: F, t30287: F, t30290: F, t30294: F, t30298: F, t30306: F, t3190: F, t5136: F, t551: F, t552: F) -> (F,) {
    let t34256 = t7601 * t9293;
    let t34258 = t25872 - 0.10401866088065122276e1 * t30252 - 0.86743646395112941038e-3 * t30254 - 0.12713391885412927226e1 * t30258 - 0.7801399566048841707e0 * t5136 * t551 * t552 * t3190 * t2719 + 0.69345773920434148506e0 * t30260 + 0.34930954652346593434e-1 * t30283 + 0.52396431978519890151e-1 * t30287 + 0.87816964854445047169e-1 * t30290 - 0.20958572791407956061e0 * t30294 - 0.1047928639570397803e1 * t30298 - 0.17465477326173296717e-1 * t30306 - 0.7801399566048841707e0 * t20286 * t10127 - 0.17465477326173296717e-1 * t34256;
    (t34258,)
}
