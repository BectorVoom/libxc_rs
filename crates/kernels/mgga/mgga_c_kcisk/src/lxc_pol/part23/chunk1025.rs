//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1025/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1025<F: Float>(t385: F, t20360: F, t20574: F, t67: F, t1279: F, t4143: F, t13776: F, t378: F, t1287: F, t19881: F, t19904: F, t2147: F, t2153: F, t340: F, t382: F, t4134: F, t4144: F, t4148: F, t6006: F, t6130: F, t6141: F, t6142: F) -> (F,) {
    let t386 = t385 < -0.66725e-1;
    let t20576 = t67 * (t20360 + t20574);
    let t20592 = t1279 * t4143;
    let t20596 = t378 * t13776;
    let t20604 = piecewise3(t386, 0.0, 10.0 / 9.0 * t340 * t20576 * t382 - 20.0 / 27.0 * t340 * t6130 * t1287 + 40.0 / 81.0 * t340 * t2147 * t4144 - 10.0 / 27.0 * t340 * t2147 * t4148 - 10.0 / 27.0 * t340 * t4134 * t2153 + 80.0 / 81.0 * t6141 * t20592 * t6006 - 280.0 / 243.0 * t6141 * t20596 * t19881 + 40.0 / 81.0 * t6141 * t6142 * t19904);
    (t20604,)
}
