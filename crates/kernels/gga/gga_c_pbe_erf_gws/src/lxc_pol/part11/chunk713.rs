//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 713/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk713<F: Float>(t12493: F, t5089: F, t11: F, t12350: F, t5002: F, t1691: F, t2678: F, t3354: F, t1642: F, t625: F, t2672: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12494 = t5089 * t12493;
    let t12495 = t11 * t12494;
    let t12497 = t5002 * t12350;
    let t12498 = t1691 * t12497;
    let t12499 = t11 * t12498;
    let t12501 = t2678 * t3354;
    let t12502 = t1691 * t12501;
    let t12503 = t11 * t12502;
    let t12505 = t1642 * t12350;
    let t12506 = t625 * t12505;
    let t12507 = t11 * t12506;
    let t12509 = t2672 * t3354;
    (t12494, t12495, t12497, t12498, t12499, t12501, t12502, t12503, t12505, t12506, t12507, t12509)
}
