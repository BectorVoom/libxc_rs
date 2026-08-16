//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 885/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk885<F: Float>(t50: F, t478: F, t9801: F, t9794: F, t9796: F, t9799: F, t9792: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t9802 = t478 * t9801;
    let t9805 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9794 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9796 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9799 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9802);
    let t9807 = t9792 / F::cast_from(2.0_f64) + t9805 / F::cast_from(2.0_f64);
    (t9802, t9807)
}
