//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1126/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1126<F: Float>(t1762: F, t224: F, t5206: F, t5569: F, t5227: F, t5376: F, t5249: F, t766: F, t1767: F, t5435: F, t1893: F, t21066: F, t5446: F, t1751: F, t5938: F, t1398: F, t1745: F, t735: F) -> (F, F, F, F, F, F, F) {
    let t21159 = 0.11407595979765752407e3 * t1762 * t5206 * t224 * t5569;
    let t21167 = t5376 * t5227;
    let t21168 = t5249 * t766 * t21167;
    let t21173 = 0.39036892681086263432e0 * t1762 * t1767 * t224 * t5435;
    let t21176 = 0.13264618753018470773e4 * t5446 * t21066 * t1893;
    let t21179 = t1751 * t5938;
    let t21183 = 0.43374325201206959368e-1 * t735 * t1398 * t1745;
    (t21159, t21167, t21168, t21173, t21176, t21179, t21183)
}
