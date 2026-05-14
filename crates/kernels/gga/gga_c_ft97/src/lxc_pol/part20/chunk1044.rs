//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1044/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1044<F: Float>(t99534: F, t99545: F, t99555: F, t99557: F, t99567: F, t99599: F, t99601: F, t99610: F, t1882: F, t25384: F, t25281: F, t8392: F, t2399: F, t6349: F, t89: F, t25301: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t99801 = 28.0 / 81.0 * t99534;
    let t99806 = t99545 / 3.0;
    let t99809 = 2.0 / 3.0 * t99555;
    let t99810 = t99557 / 18.0;
    let t99812 = 2.0 * t99567;
    let t99822 = t99599 / 6.0;
    let t99823 = 4.0 / 9.0 * t99601;
    let t99826 = 4.0 / 3.0 * t99610;
    let t99848 = t1882 * t25384;
    let t99850 = t8392 * t25281;
    let t99867 = t89 * t2399 * t6349;
    let t99885 = t1882 * t25301;
    (t99801, t99806, t99809, t99810, t99812, t99822, t99823, t99826, t99848, t99850, t99867, t99885)
}
