//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 725/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk725<F: Float>(t529: F, t1556: F, t2306: F, t41: F, t5798: F, t260: F, t338: F, t67: F, t4143: F, t6006: F, t1287: F, t1558: F, t2153: F, t2308: F, t382: F, t525: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t530 = t529 < -0.66725e-1;
    let t6426 = t2306 * t1556;
    let t6431 = t5798 * t41;
    let t6442 = t260 * t67 * t338;
    let t6443 = t41 * t4143;
    let t6444 = t6443 * t6006;
    let t6448 = piecewise3(t530, 0.0, 10.0 / 9.0 * t525 * t6431 * t382 - 10.0 / 27.0 * t525 * t2308 * t1287 - 10.0 / 27.0 * t525 * t1558 * t2153 + 40.0 / 81.0 * t6442 * t6444);
    let t6449 = t79 * t6448;
    (t6426, t6431, t6442, t6443, t6444, t6448, t6449)
}
