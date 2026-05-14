//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 252/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk252<F: Float>(t1190: F, t1191: F, t1172: F, t1161: F, t1165: F) -> (F, F, F, F) {
    let t1192 = t1190 * t1191;
    let t1194 = 1.0 * t1172 * t1192;
    let t1195 = 0.92708333333333333333e-2 * t1161;
    let t1197 = -t1195 - 0.92708333333333333333e-2 * t1165;
    (t1192, t1194, t1195, t1197)
}
