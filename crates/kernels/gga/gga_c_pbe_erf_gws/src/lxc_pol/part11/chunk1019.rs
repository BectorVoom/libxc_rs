//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1019/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1019<F: Float>(t12804: F, t17268: F, t587: F, t12809: F, t1820: F, t5125: F, t12588: F, t5175: F, t12575: F, t1630: F, t639: F, t12709: F, t626: F) -> (F, F, F, F, F) {
    let t41418 = t587 * t17268 * t12804;
    let t41421 = t1820 * t5125 * t12809;
    let t41432 = t5175 * t12588;
    let t41447 = t639 * t1630 * t12575;
    let t41459 = t12709 * t626;
    (t41418, t41421, t41432, t41447, t41459)
}
