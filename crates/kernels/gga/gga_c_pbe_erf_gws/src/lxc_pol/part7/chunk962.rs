//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 962/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk962<F: Float>(t17806: F, t5516: F, t583: F, t184: F, t202: F, t5371: F, t619: F, t5099: F, t633: F, t5029: F, t639: F, t7877: F) -> (F, F, F, F, F) {
    let t17807 = F::new(32.0) / F::new(15.0) * t17806;
    let t17808 = t5516 * t583;
    let t17809 = F::new(32.0) / F::new(15.0) * t17808;
    let t17811 = t202 * t5371 * t184;
    let t17813 = F::new(16.0) / F::new(15.0) * t17811 * t619;
    let t17815 = F::new(8.0) / F::new(15.0) * t633 * t5099;
    let t17817 = t639 * t7877 * t5029;
    (t17807, t17809, t17813, t17815, t17817)
}
