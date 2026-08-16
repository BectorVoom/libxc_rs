//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1336/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1336<F: Float>(t40135: F, t760: F, t39875: F, t39894: F, t9371: F, t39960: F, t39963: F, t39909: F, t738: F, t745: F, t9417: F, t2596: F, t39871: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40137 = F::cast_from(0.6233709278045326953e3_f64) * t760 * t40135;
    let t40165 = t39894 * t39875 * t9371;
    let t40167 = F::cast_from(0.12304822629859687989e5_f64) * t760 * t40165;
    let t40169 = t39960 * t39875 * t39963;
    let t40171 = F::cast_from(0.91082604192152556044e5_f64) * t760 * t40169;
    let t40182 = t738 * t39909 * t745;
    let t40184 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t40182;
    let t40192 = t9417 * t39875 * t745;
    let t40194 = F::cast_from(0.14035736694323150897e2_f64) * t760 * t40192;
    let t40196 = t2596 * t39871 * t745;
    (t40137, t40165, t40167, t40169, t40171, t40182, t40184, t40192, t40194, t40196)
}
