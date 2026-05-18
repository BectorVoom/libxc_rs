//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 979/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk979<F: Float>(t1672: F, t185: F, t3444: F, t2612: F, t7459: F, t10033: F, t164: F, t163: F, t169: F, t3569: F, t784: F, t3379: F, t551: F, t553: F, t837: F) -> (F, F, F, F, F) {
    let t33281 = t185 * t1672 * t3444;
    let t33298 = t2612 * t7459;
    let t33381 = t10033 * t164;
    let t33385 = t169 * t784 * t3569 * t163;
    let t33389 = t837 * t3379 * t551 * t553;
    (t33281, t33298, t33381, t33385, t33389)
}
