//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 675/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk675<F: Float>(t20336: F, t582: F, t17279: F, t17281: F, t20786: F, t20789: F, t20793: F, t20796: F, t20799: F, t20802: F, t20804: F, t462: F, t9178: F, t20352: F, t20039: F, t3506: F) -> (F, F, F, F) {
    let t20806 = t582 * t20336;
    let t20809 = -2.0 * t462 * t20786 - 2.0 * t462 * t20789 - t9178 + t17279 - 2.0 * t17281 + 2.0 / 3.0 * t462 * t20793 + 4.0 / 3.0 * t462 * t20796 - 2.0 / 3.0 * t462 * t20799 + t462 * t20802 + t462 * t20804 - t462 * t20806 / 3.0;
    let t20810 = t582 * t20352;
    let t20813 = t3506 * t20039;
    (t20806, t20809, t20810, t20813)
}
