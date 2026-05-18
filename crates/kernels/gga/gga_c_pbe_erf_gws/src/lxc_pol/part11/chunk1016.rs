//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1016/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1016<F: Float>(t12571: F, t1651: F, t587: F, t12778: F, t17252: F, t12612: F, t1620: F, t4934: F, t12616: F, t5137: F, t639: F, t10927: F, t2612: F) -> (F, F, F, F, F) {
    let t41223 = t587 * t1651 * t12571;
    let t41245 = t587 * t17252 * t12778;
    let t41297 = t1620 * t4934 * t12612;
    let t41300 = t639 * t5137 * t12616;
    let t41326 = t2612 * t10927;
    (t41223, t41245, t41297, t41300, t41326)
}
