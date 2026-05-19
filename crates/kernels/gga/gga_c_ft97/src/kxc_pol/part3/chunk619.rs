//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 619/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk619<F: Float>(t4977: F, t801: F, t274: F, t4939: F, t231: F, t278: F, t2014: F, t2394: F, t2710: F, t5242: F, t807: F, t291: F) -> (F, F, F, F, F, F) {
    let t5245 = t801 * t4977;
    let t5248 = t4939 * t274;
    let t5249 = t231 * t5248;
    let t5252 = t4939 * t278;
    let t5255 = t4977 * t278;
    let t5260 = -F::cast_from(0.11705142615505742e0_f64) * t5242 * t274 + F::cast_from(0.23410285231011484e0_f64) * t5245 * t274 - F::cast_from(0.26564305359272358183e-2_f64) * t2014 * t5249 + F::cast_from(0.319782988780431561e-1_f64) * t2710 * t5252 - F::cast_from(0.532971647967385935e-1_f64) * t807 * t5255 + F::cast_from(0.13977476158628290272e-1_f64) * t2394 * t5252;
    let t5261 = t291 * t5260;
    (t5245, t5249, t5252, t5255, t5260, t5261)
}
