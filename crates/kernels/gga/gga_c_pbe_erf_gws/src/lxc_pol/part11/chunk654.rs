//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 654/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk654<F: Float>(t329: F, t9246: F, t1146: F, t2242: F, t353: F, t858: F, t1120: F, t4442: F, t352: F, t6126: F, t6365: F, t904: F, t1112: F, t2079: F, t367: F, t6553: F, t899: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9270 = t329 * t9246;
    let t9275 = t2242 * t1146;
    let t9283 = t858 * t353;
    let t9290 = t4442 * t1120;
    let t9296 = t352 * t6126;
    let t9343 = t6365 * t904;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9425 = t899 * t6553 * t367;
    (t9270, t9275, t9283, t9290, t9296, t9343, t9385, t9386, t9425)
}
