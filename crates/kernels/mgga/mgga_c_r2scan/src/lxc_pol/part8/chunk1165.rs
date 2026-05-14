//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1165/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1165<F: Float>(t1654: F, t5251: F, t424: F, t5836: F, t5397: F, t5398: F, t608: F, t21472: F, t61: F, t1419: F, t1793: F, t5431: F, t732: F, t21594: F, t5434: F, t5293: F) -> (F, F, F, F, F, F, F, F) {
    let t22166 = t1654 * t5251;
    let t22169 = t424 * t5836;
    let t22173 = t5397 * t608 * t5398;
    let t22176 = 0.10132939716376971859e5 * t61 * t21472;
    let t22181 = t1419 * t1793;
    let t22183 = t732 * t5431;
    let t22186 = 0.5204919024144835124e0 * t5434 * t21594;
    let t22187 = t732 * t5293;
    (t22166, t22169, t22173, t22176, t22181, t22183, t22186, t22187)
}
