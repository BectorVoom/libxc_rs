//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 883/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk883<F: Float>(t1473: F, t3626: F, t169: F, t301: F, t3373: F, t366: F, t159: F, t285: F, t3379: F, t39: F, t1368: F, t281: F, t2030: F, t3685: F, t475: F, t1251: F, t1508: F, t3649: F) -> (F, F, F, F, F, F) {
    let t33770 = t1473 * t3626;
    let t33778 = t169 * t366 * t3373 * t301;
    let t33837 = t39 * t3379 * t159 * t285;
    let t33849 = t281 * t3379 * t1368 * t285;
    let t33854 = t475 * t3685 * t2030;
    let t33963 = t1508 * t3649 * t1251;
    (t33770, t33778, t33837, t33849, t33854, t33963)
}
