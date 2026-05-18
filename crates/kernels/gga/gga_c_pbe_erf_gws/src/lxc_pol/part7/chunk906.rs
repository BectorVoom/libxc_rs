//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 906/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk906<F: Float>(t16679: F, t1896: F, t587: F, t590: F, t1661: F, t1664: F, t1620: F, t5455: F, t5493: F, t1879: F, t5346: F, t2735: F, t616: F, t618: F) -> (F, F, F, F, F) {
    let t17094 = F::new(8.0) / F::new(15.0) * t587 * t590 * t1896 * t16679;
    let t17098 = F::new(4.0) / F::new(9.0) * t587 * t1661 * t1664 * t16679;
    let t17100 = t1620 * t5493 * t5455;
    let t17101 = F::new(64.0) / F::new(15.0) * t17100;
    let t17102 = t1879 * t5346;
    let t17103 = F::new(32.0) / F::new(15.0) * t17102;
    let t17105 = t616 * t2735 * t618;
    (t17094, t17098, t17101, t17103, t17105)
}
