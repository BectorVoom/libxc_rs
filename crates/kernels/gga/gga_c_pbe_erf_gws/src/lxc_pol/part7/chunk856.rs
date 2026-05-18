//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 856/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk856<F: Float>(t1666: F, t5304: F, t196: F, t5174: F, t188: F, t1804: F, t185: F, t186: F, t1: F, t3: F, t4562: F, t672: F) -> (F, F, F) {
    let t16529 = F::new(16.0) / F::new(9.0) * t5304 * t1666;
    let t16531 = F::new(1.0) / t5174 / t196;
    let t16532 = t188 * t16531;
    let t16533 = t1804 * t1804;
    let t16537 = F::new(16.0) / F::new(5.0) * t185 * t186 * t16532 * t16533;
    let t16540 = t4562 * t1 * t3 * t672;
    (t16529, t16537, t16540)
}
