//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 499/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk499<F: Float>(t231: F, t5248: F, t278: F, t4939: F, t4977: F, t2014: F, t2394: F, t2710: F, t274: F, t5242: F, t5245: F, t807: F) -> (F, F, F) {
    let t5249 = t231 * t5248;
    let t5252 = t4939 * t278;
    let t5255 = t4977 * t278;
    let t5260 = -0.11705142615505742e0 * t5242 * t274 + 0.23410285231011484e0 * t5245 * t274 - 0.26564305359272358183e-2 * t2014 * t5249 + 0.319782988780431561e-1 * t2710 * t5252 - 0.532971647967385935e-1 * t807 * t5255 + 0.13977476158628290272e-1 * t2394 * t5252;
    (t5249, t5252, t5260)
}
