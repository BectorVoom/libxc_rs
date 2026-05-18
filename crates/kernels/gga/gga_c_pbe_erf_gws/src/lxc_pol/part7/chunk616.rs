//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 616/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk616<F: Float>(t4778: F, t85: F, t4607: F, t4734: F, t4737: F, t470: F, t1396: F, t1399: F, t449: F, t456: F, t4619: F, t1392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4779 = t4778 * t85;
    let t4780 = F::new(0.19751789702565206229e-1) * t4779;
    let t4782 = t4734 * t4607 * t4737;
    let t4783 = t470 * t4782;
    let t4784 = F::new(0.1025389702100779493e4) * t4783;
    let t4785 = t1399 * t1396;
    let t4786 = F::new(0.17544670192365612213e1) * t4785;
    let t4788 = t449 * t4619 * t456;
    let t4789 = t470 * t4788;
    let t4790 = F::new(0.58482233974552040708e0) * t4789;
    let t4791 = t1399 * t1392;
    (t4779, t4780, t4782, t4783, t4784, t4785, t4786, t4788, t4789, t4790, t4791)
}
