//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 846/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk846<F: Float>(t2659: F, t586: F, t2581: F, t5312: F, t2784: F, t572: F, t418: F, t1827: F, t587: F, t2816: F, t636: F, t197: F, t589: F) -> (F, F, F, F, F, F) {
    let t7136 = t2659 * t586;
    let t7138 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7136 * t2581;
    let t7140 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5312 * t2581;
    let t7141 = t2784 * t572;
    let t7142 = t7141 * t418;
    let t7143 = t1827 * t7142;
    let t7145 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t7143;
    let t7147 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2816 * t636;
    let t7148 = t589 * t197;
    (t7136, t7138, t7140, t7145, t7147, t7148)
}
