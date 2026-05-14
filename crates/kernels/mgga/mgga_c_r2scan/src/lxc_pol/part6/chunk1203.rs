//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1203/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1203<F: Float>(t1654: F, t5251: F, t5250: F, t424: F, t5836: F, t5397: F, t5398: F, t608: F, t21472: F, t61: F, t4959: F, t697: F, t1376: F, t1721: F, t1419: F, t1793: F) -> (F, F, F, F, F, F, F, F) {
    let t22166 = t1654 * t5251;
    let t22167 = t5250 * t22166;
    let t22169 = t424 * t5836;
    let t22173 = t5397 * t608 * t5398;
    let t22176 = 0.10132939716376971859e5 * t61 * t21472;
    let t22177 = t4959 * t697;
    let t22179 = t1376 * t1721;
    let t22181 = t1419 * t1793;
    (t22166, t22167, t22169, t22173, t22176, t22177, t22179, t22181)
}
