//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 936/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk936<F: Float>(t4913: F, t5502: F, t1620: F, t1621: F, t5162: F, t649: F, t661: F, t1635: F, t5467: F, t1645: F, t5301: F, t5312: F) -> (F, F, F, F, F) {
    let t17456 = F::new(16.0) / F::new(5.0) * t4913 * t5502;
    let t17461 = F::new(16.0) / F::new(15.0) * t1620 * t1621 * t649 * t5162 * t661;
    let t17463 = F::new(16.0) / F::new(15.0) * t5467 * t1635;
    let t17465 = F::new(16.0) / F::new(9.0) * t5467 * t1645;
    let t17467 = F::new(32.0) / F::new(5.0) * t5312 * t5301;
    (t17456, t17461, t17463, t17465, t17467)
}
