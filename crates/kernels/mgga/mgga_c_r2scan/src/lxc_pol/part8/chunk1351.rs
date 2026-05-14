//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1351/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1351<F: Float>(t108: F, t111: F, t113: F, t2498: F, t2506: F, t2527: F, t3042: F, t3046: F, t3049: F, t32956: F, t32957: F, t32959: F, t32960: F, t32975: F, t32988: F, t32993: F, t33004: F, t33061: F, t491: F, t8662: F, t8668: F, t8676: F, t8679: F, t8685: F, t917: F, t95: F, t9929: F) -> (F,) {
    let t33063 = (-(t32956 + t32957 + t32959 + t32960 + t32975 + t32988 + t32993 + t33004) * t108 * t111 + 3.0 * t9929 * t491 + 9.0 * t8662 * t917 - 36.0 * t3042 * t95 * t2506 + 9.0 * t3042 * t2527 - 36.0 * t2498 * t3046 + 180.0 * t8668 * t8676 - 72.0 * t8668 * t8679 + 9.0 * t2498 * t3049 - 36.0 * t8668 * t8685 + t33061) * t113;
    (t33063,)
}
