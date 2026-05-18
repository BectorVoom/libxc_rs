//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1218/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1218<F: Float>(t2108: F, t339: F, t2080: F, t2084: F, t860: F, t2142: F, t6493: F, t20133: F, t326: F, t6094: F, t19561: F, t20134: F) -> (F, F, F) {
    let t21610 = t2108 * t339;
    let t21614 = t2080 * t2084 * t21610 * t860 / F::new(32.0);
    let t21615 = t6493 * t2142;
    let t21616 = F::new(7.0) / F::new(72.0) * t21615;
    let t21621 = t326 * t20133;
    let t21623 = t6094 * t339;
    let t21627 = t21621 * t20134 * t19561 * t21623 * t860 / F::new(96.0);
    (t21614, t21616, t21627)
}
