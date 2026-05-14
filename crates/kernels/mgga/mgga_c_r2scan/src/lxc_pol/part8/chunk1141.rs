//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1141/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1141<F: Float>(t19529: F, t19530: F, t19531: F, t19532: F, t19533: F, t19534: F, t21074: F, t21077: F, t21080: F, t21083: F, t21115: F, t5266: F, t5269: F, t19570: F, t19571: F, t19572: F, t19573: F, t19574: F, t19575: F) -> (F, F, F) {
    let t21467 = -0.78438333333333333333e1 * t21074 + 0.188252e2 * t21077 - 0.69722962962962962964e1 * t21080 + 0.24403037037037037037e2 * t21083 + t19529 + t19530 - t19531 + t19532 + t19533 + t19534;
    let t21472 = t5266 * t21115 * t5269;
    let t21478 = -0.5753888888888888889e1 * t21074 + 0.13809333333333333334e2 * t21077 - 0.51145679012345679013e1 * t21080 + 0.17900987654320987655e2 * t21083 + t19570 + t19571 - t19572 + t19573 + t19574 + t19575;
    (t21467, t21472, t21478)
}
