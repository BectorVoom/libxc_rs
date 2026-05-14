//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 371/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk371<F: Float>(t645: F, t2436: F, t2442: F, t340: F, t639: F, t642: F, sigma2: F) -> (F,) {
    let t646 = t645 < -0.66725e-1;
    let t2447 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t2436 * t642 - 10.0 / 27.0 * t340 * t639 * t2442);
    let t2448 = t2447 * sigma2;
    (t2448,)
}
