//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1131/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1131<F: Float>(t5210: F, t5967: F, t5203: F, t1762: F, t377: F, t5290: F, t5531: F, t1767: F, t5543: F, t5661: F, t5964: F, t5200: F, t1727: F, t153: F, t158: F, t171: F, t1724: F) -> (F, F, F, F, F, F, F, F) {
    let t21283 = t5967 * t5210;
    let t21287 = t5967 * t5203;
    let t21292 = 0.45630383919063009625e3 * t1762 * t377 * t5290 * t5531;
    let t21295 = 0.19263893255070628432e1 * t1762 * t1767 * t5543;
    let t21298 = 0.19263893255070628432e1 * t1762 * t1767 * t5661;
    let t21299 = t5967 * t5964;
    let t21301 = t5967 * t5200;
    let t21306 = t1727 * t1727;
    let t21309 = 0.3429168e1 / t1724 / t153 * t158 * t171 * t21306;
    (t21283, t21287, t21292, t21295, t21298, t21299, t21301, t21309)
}
