//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 976/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk976<F: Float>(t385: F, t1284: F, t26416: F, t1280: F, t1287: F, t20596: F, t2153: F, t26387: F, t26397: F, t26404: F, t26411: F, t340: F, t379: F, t382: F, t6006: F, t6130: F, t6141: F, t6142: F, t8003: F, t8011: F, t8015: F) -> (F, F) {
    let t386 = t385 < -0.66725e-1;
    let t26417 = t1284 * t26416;
    let t26422 = piecewise3(t386, 0.0, 10.0 / 9.0 * t340 * t26387 * t382 - 10.0 / 27.0 * t340 * t8003 * t1287 - 20.0 / 27.0 * t340 * t6130 * t2153 + 80.0 / 81.0 * t6141 * t26397 * t6006 + 40.0 / 81.0 * t340 * t1280 * t8011 - 280.0 / 243.0 * t6141 * t20596 * t26404 - 10.0 / 27.0 * t340 * t1280 * t8015 + 40.0 / 81.0 * t6141 * t6142 * t26411 - 10.0 / 27.0 * t340 * t379 * t26417);
    (t26417, t26422)
}
