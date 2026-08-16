//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 868/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk868<F: Float>(t32: F, t7911: F, t8991: F, t123: F, t37993: F, t532: F, t120: F, t1557: F, t2264: F, t341: F, t17: F, t8946: F, t8947: F) -> (F, F, F, F, F) {
    let t39877 = t8991 / t32 / t7911;
    let t39889 = t123 / t532 / t37993;
    let t39912 = t120 * t1557;
    let t39922 = t341 * t2264;
    let t39926 = t8946 * t8947 * t17;
    (t39877, t39889, t39912, t39922, t39926)
}
