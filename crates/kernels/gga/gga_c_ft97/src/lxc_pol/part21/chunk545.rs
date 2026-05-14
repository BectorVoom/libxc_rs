//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 545/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk545<F: Float>(t29: F, t31: F, t122: F, t170: F, t7239: F, t30: F, t25: F, t23: F, t2999: F) -> (F, F, F, F, F, F) {
    let t7905 = 1.0 / t31 / t29;
    let t7906 = t122 * t7905;
    let t7911 = 4.0 * t170 * t7239;
    let t7913 = 1.0 / t30 / t7911;
    let t7914 = t25 * t7913;
    let t7943 = t2999 * t23;
    (t7905, t7906, t7911, t7913, t7914, t7943)
}
