//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 620/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk620<F: Float>(t3664: F, t3665: F, t122: F, t515: F, t125: F, t169: F) -> (F, F, F, F) {
    let t3666 = t3664 * t3665;
    let t3668 = t515 * t122;
    let t3669 = t3668 * t125;
    let t3670 = t169 * t3669;
    (t3666, t3668, t3669, t3670)
}
