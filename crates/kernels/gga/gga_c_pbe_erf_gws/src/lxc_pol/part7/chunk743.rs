//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 743/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk743<F: Float>(t2100: F, t817: F, t2106: F, t814: F, t816: F, t322: F, t2108: F, t745: F, t1452: F, t2102: F, t2107: F, t323: F, t4867: F, t6084: F, t818: F) -> (F, F, F, F, F, F, F, F) {
    let t6086 = t2100 * t817;
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    let t6095 = F::new(1.0) / t6094;
    let t6096 = t322 * t6095;
    let t6097 = t2108 * t745;
    let t6100 = t745 * t1452;
    let t6104 = -F::new(3.0) * t1452 * t2102 + F::new(6.0) * t2107 * t6100 + F::new(6.0) * t2108 * t6089 + t323 * t6084 - t4867 * t818 - F::new(3.0) * t6086 * t745 - F::new(6.0) * t6096 * t6097;
    (t6086, t6089, t6094, t6095, t6096, t6097, t6100, t6104)
}
