//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1113/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1113<F: Float>(t5053: F, t4934: F, t21337: F, t21382: F, t21309: F, t4952: F, t6: F, t30852: F, t65693: F, t21333: F, t39: F, t4960: F) -> (F, F, F, F, F, F) {
    let t88289 = t5053 * t5053;
    let t88294 = t4934 * t4934;
    let t88310 = t21337 * t21382;
    let t88314 = t21309 * t6 * t4952;
    let t88320 = t30852 * t65693;
    let t88337 = t4960 * t39 * t21333;
    (t88289, t88294, t88310, t88314, t88320, t88337)
}
