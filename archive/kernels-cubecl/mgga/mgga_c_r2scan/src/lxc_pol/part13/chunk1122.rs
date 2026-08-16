//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1122/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1122<F: Float>(t10768: F, t8129: F, t2196: F, t24790: F, t3308: F, t2604: F, t625: F, t37637: F, t1060: F, t269: F, t783: F, t7916: F) -> (F, F, F, F, F) {
    let t39464 = t10768 * t8129;
    let t39467 = t2196 * t3308 * t24790;
    let t39469 = t2604 * t625;
    let t39470 = t37637 * t39469;
    let t39476 = t783 * t7916 * t269 * t1060;
    (t39464, t39467, t39469, t39470, t39476)
}
