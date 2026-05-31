//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 646/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk646<F: Float>(t3617: F, t797: F, t1048: F, t499: F, t2867: F, t3263: F, t3275: F, t1010: F, t3358: F, t1070: F, t2378: F, t1276: F) -> (F, F, F, F, F, F, F) {
    let t3618 = t3617 * t797;
    let t3620 = t1048 * t499 * t3618;
    let t3621 = t3620 / F::cast_from(4.0_f64);
    let t3623 = t3275 * t3263 * t2867;
    let t3624 = t3623 / F::cast_from(4.0_f64);
    let t3625 = t3358 * t1010;
    let t3627 = t2378 * t1070;
    let t3629 = t1070 * t1010;
    let t3630 = t1276 * t3629;
    (t3618, t3621, t3624, t3625, t3627, t3629, t3630)
}
