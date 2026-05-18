//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 669/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk669<F: Float>(t19278: F, t19298: F, t19301: F, t19304: F, t5: F, t5429: F, t4417: F, t7712: F, t7720: F, t1528: F, t4431: F, t4495: F, t72: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19852 = F::new(2.0) / F::new(9.0) * t19278;
    let t19857 = t19298 / F::new(9.0);
    let t19858 = F::new(2.0) / F::new(9.0) * t19301;
    let t19859 = F::new(2.0) / F::new(27.0) * t19304;
    let t19920 = t5 * t5429;
    let t19950 = t7712 * t4417;
    let t19957 = t7720 * t4417;
    let t19961 = t1528 * t4431;
    let t19965 = t72 * t4495;
    (t19852, t19857, t19858, t19859, t19920, t19950, t19957, t19961, t19965)
}
