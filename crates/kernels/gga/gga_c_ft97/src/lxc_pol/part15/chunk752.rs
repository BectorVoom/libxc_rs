//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 752/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk752<F: Float>(t1255: F, t4969: F, t835: F, t14715: F, t21947: F, t21951: F, t21955: F, t21960: F, t21964: F, t21967: F, t21971: F, t21975: F, t21984: F, t21987: F, t21991: F, t21994: F) -> (F, F) {
    let t22261 = t835 * t1255 * t4969;
    let t22275 = 2.0 * t21994 + t21971 + t21975 + 2.0 / 3.0 * t21960 - 2.0 / 3.0 * t21967 - 2.0 * t21947 - 2.0 * t21951 - 10.0 / 27.0 * t21955 + 4.0 / 3.0 * t21964 + 6.0 * t21984 - t21987 / 3.0 - 2.0 * t21991 - 4.0 / 9.0 * t14715;
    (t22261, t22275)
}
