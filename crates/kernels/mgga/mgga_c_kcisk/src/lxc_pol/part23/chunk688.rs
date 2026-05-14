//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 688/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk688<F: Float>(t385: F, t6129: F, t67: F, t339: F, t63: F, t378: F, t4143: F, t1280: F, t1287: F, t2147: F, t2153: F, t340: F, t382: F, t6006: F) -> (F, F, F, F) {
    let t386 = t385 < -0.66725e-1;
    let t6130 = t67 * t6129;
    let t6141 = t339 * t63 * t67;
    let t6142 = t378 * t4143;
    let t6147 = piecewise3(t386, 0.0, 10.0 / 9.0 * t340 * t6130 * t382 - 10.0 / 27.0 * t340 * t2147 * t1287 - 10.0 / 27.0 * t340 * t1280 * t2153 + 40.0 / 81.0 * t6141 * t6142 * t6006);
    (t6130, t6141, t6142, t6147)
}
