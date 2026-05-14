//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 855/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk855<F: Float>(t1882: F, t32622: F, t23405: F, t32719: F, t1359: F, t5973: F, t614: F, t7312: F, t32709: F, t378: F, t1389: F, t358: F, t7339: F, t32967: F, t32706: F, t5766: F) -> (F, F, F, F, F, F, F, F, F) {
    let t138367 = t1882 * t32622;
    let t138411 = t23405 * t32719;
    let t138415 = t1359 * t5973;
    let t138420 = t7312 * t614;
    let t138425 = t378 * t32709;
    let t138433 = t1389 * t358;
    let t138438 = t7339 * t614;
    let t138445 = t378 * t32967;
    let t138476 = t5766 * t32706;
    (t138367, t138411, t138415, t138420, t138425, t138433, t138438, t138445, t138476)
}
