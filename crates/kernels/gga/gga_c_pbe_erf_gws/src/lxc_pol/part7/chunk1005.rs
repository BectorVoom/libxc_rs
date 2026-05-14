//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1005/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1005<F: Float>(t20281: F, t2382: F, t6680: F, t6573: F, t810: F, t1452: F, t343: F, t874: F, t5: F, t6231: F) -> (F, F, F, F) {
    let t20282 = t2382 * t20281;
    let t20284 = t20282 * t6680 / 8.0;
    let t20285 = t6573 * t810;
    let t20291 = t1452 * t874 * t343;
    let t20296 = t5 * t6231;
    (t20284, t20285, t20291, t20296)
}
