//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 905/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk905<F: Float>(t17082: F, t17058: F, t17063: F, t17067: F, t17069: F, t17071: F, t17073: F, t17075: F, t17077: F, t17079: F, t17081: F, t1661: F, t16669: F, t5294: F, t587: F) -> (F, F, F) {
    let t17083 = F::new(16.0) / F::new(45.0) * t17082;
    let t17084 = -t17058 - t17063 + t17067 - t17069 - t17071 + t17073 - t17075 + t17077 + t17079 - t17081 - t17083;
    let t17090 = F::new(16.0) / F::new(3.0) * t587 * t1661 * t5294 * t16669;
    (t17083, t17084, t17090)
}
