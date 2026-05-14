//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 594/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk594<F: Float>(t1327: F, t5874: F, t1333: F, t2214: F, t2236: F, t3512: F, t1411: F, t1413: F, t2211: F) -> (F, F, F, F, F) {
    let t5875 = t5874 * t1327;
    let t5880 = t1333 * t2214;
    let t5882 = t3512 * t2236;
    let t5883 = t1411 * t5882;
    let t5885 = t2211 * t1413;
    (t5875, t5880, t5882, t5883, t5885)
}
