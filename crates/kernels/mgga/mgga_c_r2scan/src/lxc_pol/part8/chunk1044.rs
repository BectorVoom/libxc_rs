//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1044/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1044<F: Float>(t44: F, t51: F, t1212: F, t3002: F, t903: F, t9858: F, t9864: F, t99: F, t101: F, t1224: F, t3010: F, t906: F, t9869: F, t9874: F, zeta_threshold: F) -> (F,) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t10349 = piecewise3(t45, 0.0, -10.0 / 27.0 * t1212 * t9858 + 10.0 / 3.0 * t903 * t3002 + 5.0 / 3.0 * t99 * t9864);
    let t10357 = piecewise3(t52, 0.0, -10.0 / 27.0 * t1224 * t9869 + 10.0 / 3.0 * t906 * t3010 + 5.0 / 3.0 * t101 * t9874);
    let t10359 = t10349 / 2.0 + t10357 / 2.0;
    (t10359,)
}
