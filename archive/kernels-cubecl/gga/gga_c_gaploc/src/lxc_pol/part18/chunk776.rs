//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 776/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk776<F: Float>(t1: F, t7275: F, t787: F, t161: F, t165: F, t1835: F, t969: F, t2615: F, t2617: F, t826: F, t2679: F, t825: F) -> (F, F, F, F, F, F) {
    let t7339 = t7275 * t1;
    let t7340 = t787 * t7339;
    let t7344 = t161 * t165 * t1835;
    let t7345 = t969 * t7344;
    let t7346 = t2615 * t7345;
    let t7348 = t826 * t2617;
    let t7349 = t2615 * t7348;
    let t7351 = t826 * t2679;
    let t7352 = t825 * t7351;
    (t7339, t7340, t7344, t7346, t7349, t7352)
}
