//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1068/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1068<F: Float>(t2251: F, t2276: F, t6383: F, t6: F, t6385: F, t2306: F, t6277: F, t20504: F, t3065: F, t858: F, t8988: F, t6217: F, t6411: F, t5: F, t6439: F, t343: F) -> (F, F, F, F, F, F, F) {
    let t21399 = t2276 * t2251 * t6383;
    let t21400 = t6 * t6385;
    let t21405 = t2306 * t6277;
    let t21410 = t3065 * t858 * t20504;
    let t21412 = t8988 * t21410 / 4.0;
    let t21414 = t6217 * t6411 / 16.0;
    let t21419 = t5 * t6439;
    let t21420 = t21419 * t343;
    (t21399, t21400, t21405, t21412, t21414, t21419, t21420)
}
