//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1097/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1097<F: Float>(t12643: F, t2612: F, t10326: F, t12731: F, t2790: F, t3564: F, t40899: F, t47565: F, t47566: F, t47567: F, t47568: F, t47570: F, t47574: F, t47576: F) -> (F, F, F, F, F) {
    let t47578 = F::new(128.0) / F::new(81.0) * t2612 * t12643;
    let t47580 = F::new(16.0) / F::new(15.0) * t10326 * t12731;
    let t47582 = F::new(16.0) / F::new(15.0) * t2790 * t12731;
    let t47584 = F::new(16.0) / F::new(5.0) * t40899 * t3564;
    let t47585 = t47565 + t47566 + t47567 - t47568 + t47570 - t47574 + t47576 + t47578 + t47580 + t47582 + t47584;
    (t47578, t47580, t47582, t47584, t47585)
}
