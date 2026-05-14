//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1319/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1319<F: Float>(t51: F, t1217: F, t1224: F, t19363: F, t2517: F, t32178: F, t32181: F, t32189: F, t419: F, t476: F, t7073: F, t7076: F, t8584: F, t8616: F, t9869: F, t9874: F, t32246: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t32264 = piecewise3(t52, 0.0, -56.0 / 81.0 * t19363 * t9869 * t419 - 16.0 / 9.0 * t8616 * t1217 + 8.0 / 9.0 * t7073 * t32178 + 4.0 / 3.0 * t7076 * t32181 - 2.0 / 3.0 * t2517 * t8584 - 2.0 / 9.0 * t1224 * t9874 * t419 + 2.0 / 3.0 * t476 * t32189);
    let t32266 = t32246 / 2.0 + t32264 / 2.0;
    (t32266,)
}
