//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 860/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk860<F: Float>(t39: F, t34: F, t413: F, t1332: F, t35: F) -> (F, F, F) {
    let t16575 = F::new(72.0) * t39;
    let t16576 = t34 * t413;
    let t16577 = F::new(192.0) * t16576;
    let t16578 = t35 * t1332;
    let t16579 = F::new(120.0) * t16578;
    let t16580 = -t16575 + t16577 - t16579;
    (t16576, t16578, t16580)
}
