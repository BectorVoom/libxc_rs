//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2367/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2367<F: Float>(t39909: F, t738: F, t745: F, t760: F, t2251: F, t2609: F, t2611: F, t36: F, t716: F, t39875: F, t9417: F, t2596: F, t39871: F) -> (F, F, F, F, F, F, F) {
    let t40182 = t738 * t39909 * t745;
    let t40184 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t40182;
    let t40186 = t2611 * t2609 * t2251;
    let t40188 = t36 * t716;
    let t40192 = t9417 * t39875 * t745;
    let t40194 = F::cast_from(0.14035736694323150897e2_f64) * t760 * t40192;
    let t40196 = t2596 * t39871 * t745;
    (t40182, t40184, t40186, t40188, t40192, t40194, t40196)
}
