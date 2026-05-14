//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 282/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk282<F: Float>(t2355: F, t2493: F, t2: F, t2360: F, t2349: F, t737: F, t1934: F, t738: F, t2371: F) -> (F, F, F, F, F, F) {
    let t2494 = t2493 * t2355;
    let t2497 = t2 * t2360;
    let t2498 = t2497 * t2349;
    let t2499 = t737 * t2498;
    let t2502 = t738 * t1934;
    let t2503 = t737 * t2502;
    let t2506 = t2371 * t2;
    (t2494, t2498, t2499, t2502, t2503, t2506)
}
