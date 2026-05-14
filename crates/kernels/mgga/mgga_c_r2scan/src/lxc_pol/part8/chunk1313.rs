//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1313/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1313<F: Float>(t44: F, t10554: F, t23962: F, t2266: F, t2267: F, t9880: F, t3002: F, t415: F, t1216: F, t18806: F, t40: F, t1217: F, t1361: F, t18794: F, t2466: F, t48: F, t6976: F, t6979: F, t8561: F, t8571: F, t9858: F, t9864: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t32146 = 18.0 * t23962 * t10554;
    let t32149 = 3.0 * t2266 * t2267 * t9880;
    let t32155 = t3002 * t415;
    let t32158 = t1216 * t3002;
    let t32168 = 12.0 * t40 + 24.0 * t18806;
    let t32172 = piecewise3(t45, 0.0, 40.0 / 81.0 * t18794 * t9858 * t415 - 16.0 / 9.0 * t8561 * t1217 - 8.0 / 9.0 * t6976 * t32155 + 8.0 / 3.0 * t6979 * t32158 + 4.0 / 3.0 * t2466 * t8571 + 4.0 / 9.0 * t1361 * t9864 * t415 + 4.0 / 3.0 * t48 * t32168);
    (t32146, t32149, t32155, t32158, t32168, t32172)
}
