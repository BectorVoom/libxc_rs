//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 912/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk912<F: Float>(t51: F, t476: F, t8584: F, t1217: F, t2517: F, t419: F, t8616: F, t8621: F, t8615: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t8624 = t476 * t8584;
    let t8627 = piecewise3(t52, 0.0, 8.0 / 27.0 * t8616 * t419 + 8.0 / 9.0 * t2517 * t1217 - 2.0 / 9.0 * t8621 * t419 + 2.0 / 3.0 * t8624);
    let t8629 = t8615 / 2.0 + t8627 / 2.0;
    (t8629,)
}
