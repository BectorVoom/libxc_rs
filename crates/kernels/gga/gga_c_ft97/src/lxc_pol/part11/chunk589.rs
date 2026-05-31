//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 589/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk589<F: Float>(t370: F, t8183: F, t27: F, t89: F, t10: F, t3050: F, t83: F, t1636: F, t433: F, t1756: F, t375: F, t7804: F, t7809: F, t7813: F, t7817: F, t7820: F, t7822: F, t7827: F, t7831: F) -> (F, F, F, F, F, F) {
    let t8184 = t370 * t8183;
    let t8186 = t89 * t27 * t8184;
    let t8189 = t10 * t3050 * t83;
    let t8190 = F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t8189;
    let t8192 = t89 * t1636 * t433;
    let t8195 = t89 * t375 * t1756;
    let t8197 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7804 - t7809 / F::cast_from(9.0_f64) + t7813 / F::cast_from(6.0_f64) + t7817 / F::cast_from(6.0_f64) - t7820 / F::cast_from(9.0_f64) - t7822 / F::cast_from(9.0_f64) - t7827 / F::cast_from(3.0_f64) - t7831 / F::cast_from(3.0_f64) - t8186 / F::cast_from(6.0_f64) - t8190 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8192 + t8195 / F::cast_from(6.0_f64);
    (t8184, t8186, t8189, t8192, t8195, t8197)
}
