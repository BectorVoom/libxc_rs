//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 742/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk742<F: Float>(t1056: F, t3797: F, t9461: F, t1339: F, t1405: F, t2722: F, t415: F, t1413: F, t454: F) -> (F, F, F, F, F, F) {
    let t9462 = t3797 * t1056;
    let t9463 = t9461 * t9462;
    let t9464 = t1339 * t9463;
    let t9466 = t1405 * t2722;
    let t9467 = t415 * t9466;
    let t9469 = t454 * t1413;
    (t9462, t9463, t9464, t9466, t9467, t9469)
}
