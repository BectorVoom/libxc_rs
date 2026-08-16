//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2782/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2782<F: Float>(t40169: F, t760: F, t2523: F, t9372: F, t39909: F, t738: F, t745: F, t2251: F, t2609: F, t2611: F, t36: F, t716: F) -> (F, F, F, F, F, F) {
    let t40171 = F::cast_from(0.91082604192152556044e5_f64) * t760 * t40169;
    let t40172 = t2523 * t9372;
    let t40182 = t738 * t39909 * t745;
    let t40184 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t40182;
    let t40186 = t2611 * t2609 * t2251;
    let t40188 = t36 * t716;
    (t40171, t40172, t40182, t40184, t40186, t40188)
}
