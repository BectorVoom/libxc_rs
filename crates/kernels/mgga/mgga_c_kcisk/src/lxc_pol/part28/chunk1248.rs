//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1248/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1248<F: Float>(t2454: F, t2628: F, t20: F, t2801: F, t24511: F, t2805: F, t1586: F) -> (F, F, F, F, F) {
    let t35408 = t2628 * t2454;
    let t35409 = t35408 * t20;
    let t35410 = t2801 * t35409;
    let t35415 = t2805 * t24511;
    let t35416 = t1586 * t35415;
    (t35408, t35409, t35410, t35415, t35416)
}
