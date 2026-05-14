//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1420/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1420<F: Float>(t26838: F, t21976: F, t21985: F, t21988: F, t22000: F, t22512: F, t22513: F, t22515: F, t22517: F, t22521: F, t22523: F, t22528: F, t236: F, t23893: F, t26835: F, t76: F) -> (F,) {
    let t26839 = 0.5143752e0 * t26838;
    let t26847 = -t21976 - t21985 + 0.80040858019733333331e-2 * t26835 + t26839 - t21988 + 0.5848223622634646207e0 * t23893 * t76 * t236 + t22000 - t22512 - 0.11696447245269292414e1 * t22513 + 0.17315859105681463759e2 * t22515 - 0.70178683471615754484e1 * t22517 + t22521 - t22523 + 0.48618743904e1 * t22528;
    (t26847,)
}
