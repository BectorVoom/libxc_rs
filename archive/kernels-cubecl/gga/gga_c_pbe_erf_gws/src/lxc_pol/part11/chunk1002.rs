//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1002/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1002<F: Float>(t1114: F, t38375: F, t825: F, t2365: F, t9847: F, t3916: F, t6159: F, t2503: F, t8746: F, t3047: F, t3052: F, t26755: F, t3733: F) -> (F, F, F, F, F, F, F) {
    let t39466 = t1114 * t38375 * t825;
    let t39470 = t1114 * t9847 * t2365;
    let t39475 = t3916 * t6159;
    let t39490 = t8746 * t2503;
    let t39510 = t8746 * t3047;
    let t39521 = t8746 * t3052;
    let t39523 = t26755 * t3733;
    (t39466, t39470, t39475, t39490, t39510, t39521, t39523)
}
