//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 968/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk968<F: Float>(t40042: F, t30583: F, t30593: F, t1044: F, t1620: F, t41690: F, t7216: F, t32019: F, t3403: F, t30660: F, t40696: F, t950: F, t7062: F, t7069: F, t23109: F, t30666: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t47546 = 16.0 / 45.0 * t40042;
    let t47547 = 32.0 / 135.0 * t30583;
    let t47548 = 16.0 / 45.0 * t30593;
    let t47552 = 32.0 / 5.0 * t1620 * t7216 * t41690 * t1044;
    let t47554 = 32.0 / 15.0 * t32019 * t3403;
    let t47555 = 64.0 / 135.0 * t30660;
    let t47556 = t40696 * t950;
    let t47559 = 16.0 / 9.0 * t7062 * t7069 * t47556;
    let t47560 = 128.0 / 405.0 * t23109;
    let t47561 = 16.0 / 45.0 * t30666;
    (t47546, t47547, t47548, t47552, t47554, t47555, t47556, t47559, t47560, t47561)
}
