//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 919/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk919<F: Float>(t109: F, t111: F, t2498: F, t2504: F, t2506: F, t2527: F, t3042: F, t3046: F, t3049: F, t486: F, t491: F, t8662: F, t8668: F, t8676: F, t8679: F, t8685: F, t8688: F, t915: F, t917: F) -> (F,) {
    let t8691 = 3.0 * t109 * t8688 - t8662 * t111 + 6.0 * t2498 * t917 + 60.0 * t2504 * t8676 - 24.0 * t2504 * t8679 - 12.0 * t2504 * t8685 - 24.0 * t8668 * t2506 + 6.0 * t915 * t2527 + 3.0 * t3042 * t491 - 12.0 * t486 * t3046 + 3.0 * t486 * t3049;
    (t8691,)
}
