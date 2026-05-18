//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 924/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk924<F: Float>(t1550: F, t40331: F, t25441: F, t8410: F, t5016: F, t8542: F, t2289: F, t7939: F, t2323: F, t638: F, t7184: F, t2412: F, t7905: F) -> (F, F, F, F, F, F) {
    let t40332 = t1550 * t40331;
    let t40335 = t25441 * t8410;
    let t40337 = t5016 * t8542;
    let t40339 = t7939 * t2289;
    let t40343 = t638 * t7184 * t2323;
    let t40345 = t2412 * t7905;
    (t40332, t40335, t40337, t40339, t40343, t40345)
}
