//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 828/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk828<F: Float>(t1076: F, t3776: F, t3373: F, t11318: F, t12381: F, t13156: F, t2107: F, t3030: F, t323: F, t6096: F, t818: F, t9150: F) -> (F, F, F) {
    let t13164 = t3776 * t1076;
    let t13167 = t1076 * t3373;
    let t13171 = -F::new(3.0) * t1076 * t11318 - t12381 * t818 + t13156 * t323 - F::new(6.0) * t13164 * t6096 + F::new(6.0) * t13167 * t2107 - F::new(3.0) * t3030 * t3373 + F::new(6.0) * t3776 * t9150;
    (t13164, t13167, t13171)
}
