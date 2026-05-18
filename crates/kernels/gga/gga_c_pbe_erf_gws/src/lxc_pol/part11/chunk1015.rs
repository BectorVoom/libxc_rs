//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1015/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1015<F: Float>(t12659: F, t1820: F, t5018: F, t3346: F, t995: F, t1022: F, t3354: F, t12829: F, t679: F, t12596: F, t17139: F, t10930: F, t7527: F) -> (F, F, F, F, F, F) {
    let t41074 = t1820 * t5018 * t12659;
    let t41095 = t3346 * t995;
    let t41133 = t3354 * t1022;
    let t41184 = t12829 * t679;
    let t41208 = t17139 * t12596;
    let t41218 = t7527 * t10930;
    (t41074, t41095, t41133, t41184, t41208, t41218)
}
