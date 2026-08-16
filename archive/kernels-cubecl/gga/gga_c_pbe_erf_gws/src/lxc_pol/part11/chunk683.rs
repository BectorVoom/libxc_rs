//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 683/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk683<F: Float>(t837: F, t991: F, t551: F, t553: F, t1052: F, t163: F, t169: F, t784: F, t164: F, t3013: F, t2519: F, t547: F) -> (F, F, F, F, F) {
    let t8465 = t837 * t991;
    let t8467 = t8465 * t551 * t553;
    let t8471 = t169 * t784 * t1052 * t163;
    let t8474 = t3013 * t164;
    let t8478 = t2519 * t547;
    (t8465, t8467, t8471, t8474, t8478)
}
