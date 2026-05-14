//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 485/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk485<F: Float>(t1299: F, t382: F, t143: F, t1055: F, t1404: F, t1413: F) -> (F, F, F, F) {
    let t3494 = t1299 * t382;
    let t3499 = 2.0 * t143;
    let t3500 = 2.0 * t1055;
    let t3507 = t1404 * t1413;
    (t3494, t3499, t3500, t3507)
}
