//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 797/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk797<F: Float>(t5009: F, t2526: F, t288: F, t2858: F, t481: F, t5012: F, t5015: F, t4827: F, t4839: F, t4842: F, t4996: F, t5000: F, t5004: F, t5008: F, t5020: F, t6798: F) -> (F, F, F, F) {
    let t7015 = F::cast_from(0.5848223622634646207e0_f64) * t5009;
    let t7016 = t288 * t2526;
    let t7018 = t2858 * t7016 * t481;
    let t7019 = F::new(12.0) * t7018;
    let t7020 = F::cast_from(0.18311447306006545054e-3_f64) * t5012;
    let t7021 = F::cast_from(0.4883052614935078681e-3_f64) * t5015;
    let t7022 = F::new(2.0) * t6798 - t4996 + t5000 + t5004 + t5008 + t7015 - t7019 + t4827 - t4839 + t7020 - t7021 + t5020 - t4842;
    (t7015, t7020, t7021, t7022)
}
