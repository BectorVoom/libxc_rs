//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 922/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk922<F: Float>(t1627: F, t5490: F, t5225: F, t1640: F, t16973: F, t5401: F, t639: F, t4913: F, t5506: F, t16669: F, t5008: F, t587: F, t590: F) -> (F, F, F, F, F) {
    let t17285 = F::new(16.0) / F::new(5.0) * t1627 * t5490;
    let t17287 = F::new(32.0) / F::new(15.0) * t1627 * t5225;
    let t17291 = F::new(16.0) / F::new(3.0) * t639 * t1640 * t5401 * t16973;
    let t17293 = F::new(16.0) / F::new(5.0) * t4913 * t5506;
    let t17297 = F::new(32.0) / F::new(15.0) * t587 * t590 * t5008 * t16669;
    (t17285, t17287, t17291, t17293, t17297)
}
