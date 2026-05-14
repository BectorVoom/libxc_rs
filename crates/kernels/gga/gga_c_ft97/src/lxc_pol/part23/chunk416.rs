//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 416/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk416<F: Float>(t5073: F, t729: F, t762: F, t1091: F, t1175: F, t724: F, t265: F, t4973: F, t2594: F, t4965: F, t1154: F) -> (F, F, F, F, F) {
    let t5075 = t729 * t762 * t5073;
    let t5079 = t724 * t1175 * t1091;
    let t5083 = t724 * t265 * t4973;
    let t5087 = t2594 * t265 * t4965;
    let t5092 = t1154 * t1154;
    (t5075, t5079, t5083, t5087, t5092)
}
