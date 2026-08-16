//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1227/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1227<F: Float>(t20916: F, t20919: F, t20921: F, t20964: F, t20969: F, t21039: F, t21064: F, t21068: F, t21115: F, t21123: F, t21127: F, t21155: F, t21158: F, t21174: F, t21176: F, t21183: F, t21187: F, t21191: F, t21222: F, t21224: F, t21231: F, t21239: F) -> (F, F) {
    let t21702 = t20916 + t20919 - t20921 + t20964 + t20969 - t21039 - t21064 + t21068 + t21115 - t21123 - t21127;
    let t21704 = t21155 - t21158 + t21174 + t21176 + t21183 + t21187 - t21191 - t21222 + t21224 + t21231 - t21239;
    (t21702, t21704)
}
